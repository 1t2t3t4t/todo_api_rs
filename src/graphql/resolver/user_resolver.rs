use async_graphql::{Context, Object};

use crate::graphql::model::user::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn rand_user<'a>(&self, ctx: &Context<'a>) -> User {
        User {
            first_name: "Abc".to_string(),
            last_name: "Bcd".to_string(),
            age: 18,
            bio: None
        }
    }
}