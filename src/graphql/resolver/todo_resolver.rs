use crate::graphql::model::todo::Todo;
use crate::graphql::resolver::get_database;
use async_graphql::{Context, Object};
use chrono::Utc;

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    async fn todos<'a>(&self, ctx: &Context<'a>) -> Vec<Todo> {
        let db = get_database(ctx);
        db.get_all_todo()
    }

    async fn count_todos<'a>(&self, ctx: &Context<'a>) -> usize {
        let db = get_database(ctx);
        db.get_all_todo().len()
    }
}

#[derive(Default)]
pub struct TodoMutation;

#[Object]
impl TodoMutation {
    async fn save_todo<'a>(&self, ctx: &Context<'a>) -> Todo {
        let db = get_database(ctx);
        let todo = Todo {
            name: "Test".to_string(),
            description: Some("Hi".to_string()),
            deadline: Utc::now(),
        };
        db.save_todo(todo.clone());
        todo
    }
}
