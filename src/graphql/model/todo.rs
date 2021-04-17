use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub name: String,
    pub description: Option<String>,
    pub deadline: DateTime<Utc>,
    pub rank: i32
}
