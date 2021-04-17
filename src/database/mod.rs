use crate::model::todo::Todo;
use std::sync::Mutex;
use std::marker::PhantomData;

pub trait Database: Send + Sync {
    fn save_todo(&self, todo: Todo);
    fn get_all_todo(&self) -> Vec<Todo>;
    fn get_todo_with_rank(&self, rank: i32) -> Vec<Todo>;
}

#[derive(Default)]
pub struct FileSystemDatabase {
    fs_mutex: Mutex<()>,
}

const FILE_PATH: &str = "db";

enum Entity {
    Todo,
}

impl Entity {
    fn name(&self) -> &str {
        match self {
            Self::Todo => "todo",
        }
    }
}

fn entity_file_path(entity: Entity) -> String {
    let p = std::path::Path::new(FILE_PATH);
    if !p.exists() {
        std::fs::create_dir(p).expect("Cannot create dir that does not exist");
    }
    format!("{}/{}.json", FILE_PATH, entity.name())
}

impl FileSystemDatabase {
    fn get_todos(&self) -> Vec<Todo> {
        let _guard = self.fs_mutex.lock();
        let file_content = std::fs::read_to_string(entity_file_path(Entity::Todo));
        if let Ok(todos_json) = file_content {
            let todos = serde_json::from_str::<Vec<Todo>>(&todos_json);
            todos.unwrap_or_default()
        } else {
            vec![]
        }
    }

    fn save_todos(&self, todos: Vec<Todo>) {
        let _guard = self.fs_mutex.lock();
        let todos_json = serde_json::to_string(&todos);
        if let Ok(todos_json) = todos_json {
            std::fs::write(entity_file_path(Entity::Todo), &todos_json).unwrap();
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

    fn get_todo_with_rank(&self, rank: i32) -> Vec<Todo> {
        self.get_todos()
            .iter()
            .filter(|x| x.rank == rank)
            .map(|x| x.clone())
            .collect()
    }
}
