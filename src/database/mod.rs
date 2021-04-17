use crate::graphql::model::todo::Todo;

pub trait Database {
    fn save_todo(&self, todo: Todo);
}

pub struct FileSystemDatabase;

const FILE_PATH: &str = "todos.json";

impl FileSystemDatabase {
    fn get_todos(&self) -> Vec<Todo> {
        let file_content = std::fs::read_to_string(FILE_PATH);
        // if let Some(todos_json) = file_content {
        //     let todos = serde::Deserialize::deserialize::<Vec<Todo>>(todos_json);
        // } else {
        //     vec![]
        // }
        vec![]
    }
}

impl Database for FileSystemDatabase {
    fn save_todo(&self, todo: Todo) {

    }
}
