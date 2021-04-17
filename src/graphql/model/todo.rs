use async_graphql::SimpleObject;
use chrono::{Utc, DateTime};

#[derive(SimpleObject)]
pub struct Todo {
    pub name: String,
    pub description: Option<String>,
    pub deadline: DateTime<Utc>
}