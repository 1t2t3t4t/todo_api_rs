use async_graphql::SimpleObject;

#[derive(SimpleObject, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub bio: Option<String>,
}
