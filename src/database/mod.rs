use crate::graphql::model::todo::Todo;
use std::sync::Mutex;

pub trait Database: Send + Sync {
    fn save_todo(&self, todo: Todo);
    fn get_all_todo(&self) -> Vec<Todo>;
}

#[derive(Default)]
pub struct FileSystemDatabase {
    fs_mutex: Mutex<()>,
}

const FILE_PATH: &str = "todos.json";

impl FileSystemDatabase {
    fn get_todos(&self) -> Vec<Todo> {
        let _guard = self.fs_mutex.lock();
        let file_content = std::fs::read_to_string(FILE_PATH);
        if let Ok(todos_json) = file_content {
            let todos = serde_json::from_str::<Vec<Todo>>(&todos_json);
            if let Ok(todos) = todos {
                todos
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    fn save_todos(&self, todos: Vec<Todo>) {
        let _guard = self.fs_mutex.lock();
        let todos_json = serde_json::to_string(&todos);
        if let Ok(todos_json) = todos_json {
            std::fs::write(FILE_PATH, &todos_json).unwrap();
        }
    }
}

impl Database for FileSystemDatabase {
    fn save_todo(&self, todo: Todo) {
        let mut todos = self.get_todos();
        todos.push(todo);
        self.save_todos(todos);
    }

    fn get_all_todo(&self) -> Vec<Todo> {
        self.get_todos()
    }
}
