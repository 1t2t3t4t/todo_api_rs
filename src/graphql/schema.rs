use async_graphql::{EmptySubscription, MergedObject};

use super::resolver::user_resolver::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
