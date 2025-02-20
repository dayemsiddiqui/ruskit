use crate::app::entities::User;
use crate::framework::database::factory::Factory;
use fake::{Fake, Faker};

pub struct UserFactory;

impl Factory for User {
    fn definition() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: 0,
            name: Faker.fake(),
            email: Faker.fake(),
            created_at: now.to_string(),
            updated_at: now.to_string(),
        }
    }
} 