use crate::graphql::model::todo::Todo;
use crate::graphql::resolver::get_database;
use async_graphql::{Context, Object, InputObject};
use chrono::{Utc, DateTime};

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

#[derive(InputObject)]
struct SaveTodoInput {
    name: String,
    description: Option<String>,
    deadline: DateTime<Utc>,
    rank: i32,
}

impl Into<Todo> for SaveTodoInput {
    fn into(self) -> Todo {
        Todo {
            name: self.name,
            description: self.description,
            deadline: self.deadline,
            rank: self.rank
        }
    }
}

#[Object]
impl TodoMutation {
    async fn save_todo<'a>(&self, ctx: &Context<'a>, input: SaveTodoInput) -> Todo {
        let db = get_database(ctx);
        let todo: Todo = input.into();
        db.save_todo(todo.clone());
        todo
    }
}
