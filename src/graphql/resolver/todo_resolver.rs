use async_graphql::{Object, Context};
use crate::graphql::model::todo::Todo;
use chrono::Utc;

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    async fn todo(&self) -> Todo {
        Todo {
            name: "Test".to_string(),
            description: Some("Hi".to_string()),
            deadline: Utc::now()
        }
    }
}

#[derive(Default)]
pub struct TodoMutation;

#[Object]
impl TodoMutation {
    async fn save_todo<'a>(&self, ctx: &Context<'a>) -> Todo {
        Todo {
            name: "Test".to_string(),
            description: Some("Hi".to_string()),
            deadline: Utc::now()
        }
    }
}