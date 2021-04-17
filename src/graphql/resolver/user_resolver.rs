use async_graphql::Object;

use crate::graphql::model::user::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn rand_user(&self) -> User {
        User {
            first_name: "Abc".to_string(),
            last_name: "Bcd".to_string(),
            age: 18,
            bio: None
        }
    }
}