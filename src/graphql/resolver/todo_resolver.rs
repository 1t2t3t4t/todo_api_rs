use crate::graphql::resolver::get_database;
use crate::model::todo::Todo;
use async_graphql::{Context, InputObject, Object};
use chrono::{DateTime, Utc};

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

    async fn todo_with_rank<'a>(&self, ctx: &Context<'a>, rank: i32) -> Vec<Todo> {
        get_database(ctx).get_todo_with_rank(rank)
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
            rank: self.rank,
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
