use crate::config::Config;
use crate::models::{Payment, User};
use crate::models::payment::PaymentStatus;
use crate::services::mpesa::{initiate_stk_push, MpesaCallback};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InitiatePaymentRequest {
    pub user_id: String,
    pub phone_number: String,
}

#[derive(Serialize)]
pub struct InitiatePaymentResponse {
    pub checkout_request_id: String,
    pub message: String,
}

#[post("/pay/mpesa/initiate")]
pub async fn initiate_payment(
    db: web::Data<Database>,
    config: web::Data<Config>,
    req: web::Json<InitiatePaymentRequest>,
) -> impl Responder {
    let user_id = match ObjectId::parse_str(&req.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(doc! {"error": "Invalid user_id"}),
    };

    // Verify user exists
    let users_collection = db.collection::<User>("users");
    if users_collection.find_one(doc! {"_id": user_id}).await.unwrap().is_none() {
        return HttpResponse::NotFound().json(doc! {"error": "User not found"});
    }

    // Initiate STK Push (100 KES for 24hr VIP)
    let amount = 100;
    let stk_response = match initiate_stk_push(&config, &req.phone_number, amount).await {
        Ok(resp) => resp,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(doc! {"error": format!("M-Pesa error: {}", e)})
        }
    };

    // Save payment record
    let payment = Payment {
        id: None,
        user_id,
        phone_number: req.phone_number.clone(),
        amount: amount as f64,
        mpesa_receipt: None,
        checkout_request_id: stk_response.checkout_request_id.clone(),
        status: PaymentStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let payments_collection = db.collection::<Payment>("payments");
    if let Err(_) = payments_collection.insert_one(&payment).await {
        return HttpResponse::InternalServerError().json(doc! {"error": "Failed to save payment"});
    }

    HttpResponse::Ok().json(InitiatePaymentResponse {
        checkout_request_id: stk_response.checkout_request_id,
        message: "Check your phone for M-Pesa prompt".to_string(),
    })
}

#[post("/pay/mpesa/callback")]
pub async fn mpesa_callback(
    db: web::Data<Database>,
    callback: web::Json<MpesaCallback>,
) -> impl Responder {
    let stk_callback = &callback.body.stk_callback;
    let checkout_request_id = &stk_callback.checkout_request_id;

    let payments_collection = db.collection::<Payment>("payments");
    
    // Find payment record
    let payment = match payments_collection
        .find_one(doc! {"checkout_request_id": checkout_request_id})
        .await
    {
        Ok(Some(p)) => p,
        _ => return HttpResponse::Ok().json(doc! {"ResultCode": 0, "ResultDesc": "Accepted"}),
    };

    // Check if payment was successful
    if stk_callback.result_code == 0 {
        // Extract M-Pesa receipt
        let mpesa_receipt = stk_callback
            .callback_metadata
            .as_ref()
            .and_then(|meta| {
                meta.item
                    .iter()
                    .find(|item| item.name == "MpesaReceiptNumber")
                    .and_then(|item| item.value.as_str().map(|s| s.to_string()))
            });

        // Update payment status
        payments_collection
            .update_one(
                doc! {"checkout_request_id": checkout_request_id},
                doc! {
                    "$set": {
                        "status": "Completed",
                        "mpesa_receipt": mpesa_receipt.clone(),
                        "updated_at": mongodb::bson::DateTime::now(),
                    }
                },
            )
            .await
            .ok();

        // Upgrade user to VIP for 24 hours
        let users_collection = db.collection::<User>("users");
        let vip_expires = Utc::now() + Duration::hours(24);

        users_collection
            .update_one(
                doc! {"_id": payment.user_id},
                doc! {
                    "$set": {
                        "is_vip": true,
                        "vip_expires_at": mongodb::bson::DateTime::from_system_time(vip_expires.into()),
                        "updated_at": mongodb::bson::DateTime::now(),
                    }
                },
            )
            .await
            .ok();
    } else {
        // Payment failed
        payments_collection
            .update_one(
                doc! {"checkout_request_id": checkout_request_id},
                doc! {
                    "$set": {
                        "status": "Failed",
                        "updated_at": mongodb::bson::DateTime::now(),
                    }
                },
            )
            .await
            .ok();
    }

    HttpResponse::Ok().json(doc! {"ResultCode": 0, "ResultDesc": "Accepted"})
}
