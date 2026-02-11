use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub phone_number: String,
    pub email: Option<String>,
    pub is_vip: bool,
    pub vip_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(phone_number: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            phone_number,
            email: None,
            is_vip: false,
            vip_expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_vip_active(&self) -> bool {
        if !self.is_vip {
            return false;
        }
        if let Some(expires) = self.vip_expires_at {
            expires > Utc::now()
        } else {
            false
        }
    }
}
