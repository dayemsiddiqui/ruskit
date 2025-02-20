use crate::app::entities::User;
use crate::framework::database::factory::Factory;
use fake::{Fake, Faker};

pub struct UserFactory;

impl Factory for User {
    fn definition() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        User {
            id: 0,
            name: Faker.fake(),
            email: Faker.fake(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
} 