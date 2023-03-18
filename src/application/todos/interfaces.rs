use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    application::common::{
        error::CommonError,
        query::{QueryParams, DEFAULT_LIMIT, DEFAULT_OFFSET},
        result::{RepositoryResult, ResultPaging},
    },
    domain::todos::entities::{CreateTodo, Todo},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for TodoQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<Todo>;
    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>>;
    async fn get(&self, todo_id: i32) -> RepositoryResult<Todo>;
    async fn delete(&self, todo_id: i32) -> RepositoryResult<()>;
}

#[async_trait]
pub trait TodoService: Send + Sync {
    async fn create(&self, todo: CreateTodo) -> Result<Todo, CommonError>;
    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError>;
    async fn get(&self, todo_id: i32) -> Result<Todo, CommonError>;
    async fn delete(&self, todo_id: i32) -> Result<(), CommonError>;
}
