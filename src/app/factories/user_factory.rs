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

        println!("Generating fake data...");
        let name = Name().fake::<String>();
        let email = SafeEmail().fake::<String>();
        println!("Generated name: {}, email: {}", name, email);

        let data = json!({
            "name": name,
            "email": email,
            "created_at": now,
            "updated_at": now
        });
        println!("Generated data: {:?}", data);
        data
    }
} 