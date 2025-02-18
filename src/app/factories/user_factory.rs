use crate::app::models::User;
use crate::framework::database::factory::Factory;
use fake::{faker::internet::en::*, faker::name::en::*, Fake};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

impl Factory for User {
    fn definition() -> serde_json::Value {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        json!({
            "name": Name().fake::<String>(),
            "email": FreeEmail().fake::<String>(),
            "created_at": now,
            "updated_at": now
        })
    }
} 