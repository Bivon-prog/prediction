use crate::config::Config;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use base64::Engine;

#[derive(Debug, Serialize)]
struct StkPushRequest {
    #[serde(rename = "BusinessShortCode")]
    business_short_code: String,
    #[serde(rename = "Password")]
    password: String,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "TransactionType")]
    transaction_type: String,
    #[serde(rename = "Amount")]
    amount: String,
    #[serde(rename = "PartyA")]
    party_a: String,
    #[serde(rename = "PartyB")]
    party_b: String,
    #[serde(rename = "PhoneNumber")]
    phone_number: String,
    #[serde(rename = "CallBackURL")]
    callback_url: String,
    #[serde(rename = "AccountReference")]
    account_reference: String,
    #[serde(rename = "TransactionDesc")]
    transaction_desc: String,
}

#[derive(Debug, Deserialize)]
pub struct StkPushResponse {
    #[serde(rename = "MerchantRequestID")]
    pub merchant_request_id: String,
    #[serde(rename = "CheckoutRequestID")]
    pub checkout_request_id: String,
    #[serde(rename = "ResponseCode")]
    pub response_code: String,
    #[serde(rename = "ResponseDescription")]
    pub response_description: String,
}

#[derive(Debug, Deserialize)]
pub struct MpesaCallback {
    #[serde(rename = "Body")]
    pub body: CallbackBody,
}

#[derive(Debug, Deserialize)]
pub struct CallbackBody {
    #[serde(rename = "stkCallback")]
    pub stk_callback: StkCallback,
}

#[derive(Debug, Deserialize)]
pub struct StkCallback {
    #[serde(rename = "MerchantRequestID")]
    pub merchant_request_id: String,
    #[serde(rename = "CheckoutRequestID")]
    pub checkout_request_id: String,
    #[serde(rename = "ResultCode")]
    pub result_code: i32,
    #[serde(rename = "ResultDesc")]
    pub result_desc: String,
    #[serde(rename = "CallbackMetadata")]
    pub callback_metadata: Option<CallbackMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackMetadata {
    #[serde(rename = "Item")]
    pub item: Vec<CallbackItem>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: serde_json::Value,
}

async fn get_access_token(config: &Config) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let auth = format!("{}:{}", config.mpesa_consumer_key, config.mpesa_consumer_secret);
    let encoded = base64::engine::general_purpose::STANDARD.encode(auth.as_bytes());

    let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", config.mpesa_base_url);

    #[derive(Deserialize)]
    struct TokenResponse {
        access_token: String,
    }

    let response = client
        .get(&url)
        .header("Authorization", format!("Basic {}", encoded))
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(response.access_token)
}

pub async fn initiate_stk_push(
    config: &Config,
    phone_number: &str,
    amount: u32,
) -> Result<StkPushResponse, Box<dyn Error>> {
    let access_token = get_access_token(config).await?;
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    
    let password_string = format!(
        "{}{}{}",
        config.mpesa_shortcode, config.mpesa_passkey, timestamp
    );
    let password = base64::engine::general_purpose::STANDARD.encode(password_string.as_bytes());

    let request = StkPushRequest {
        business_short_code: config.mpesa_shortcode.clone(),
        password,
        timestamp,
        transaction_type: "CustomerPayBillOnline".to_string(),
        amount: amount.to_string(),
        party_a: phone_number.to_string(),
        party_b: config.mpesa_shortcode.clone(),
        phone_number: phone_number.to_string(),
        callback_url: config.mpesa_callback_url.clone(),
        account_reference: "BetSure VIP".to_string(),
        transaction_desc: "24hr VIP Access".to_string(),
    };

    let client = Client::new();
    let url = format!("{}/mpesa/stkpush/v1/processrequest", config.mpesa_base_url);

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&request)
        .send()
        .await?
        .json::<StkPushResponse>()
        .await?;

    Ok(response)
}
