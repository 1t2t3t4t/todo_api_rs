use std::sync::Arc;

use async_graphql::Context;

use crate::database::Database;

pub mod todo_resolver;
pub mod user_resolver;

fn get_database(ctx: &Context) -> Arc<dyn Database> {
    ctx.data::<Arc<dyn Database>>().expect("Database").clone()
}
