use async_graphql::{EmptySubscription, MergedObject};

use super::resolver::user_resolver::UserQuery;
use crate::graphql::resolver::todo_resolver::{TodoQuery, TodoMutation};

#[derive(MergedObject, Default)]
pub struct RootQuery(UserQuery, TodoQuery);

#[derive(MergedObject, Default)]
pub struct RootMutation(TodoMutation);

pub type TodoSchema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;

