use crate::app::entities::User;
use crate::framework::database::factory::Factory;
use fake::{Fake, Faker};

pub struct UserFactory;

impl Factory for User {
    fn definition() -> Self {
        User {
            id: 0,
            name: Faker.fake(),
            email: Faker.fake(),
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }
} 