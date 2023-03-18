use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    application::{
        common::{error::CommonError, result::ResultPaging},
        todos::interfaces::{TodoQueryParams, TodoRepository, TodoService},
    },
    domain::todos::entities::{CreateTodo, Todo},
};

#[derive(Clone)]
pub struct TodoServiceImpl {
    pub repository: Arc<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        TodoServiceImpl { repository }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn create(&self, todo: CreateTodo) -> Result<Todo, CommonError> {
        let mut cloned = todo.clone();
        self.repository
            .create(&mut cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, todo_id: i32) -> Result<Todo, CommonError> {
        self.repository
            .get(todo_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, todo_id: i32) -> Result<(), CommonError> {
        self.repository
            .delete(todo_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
