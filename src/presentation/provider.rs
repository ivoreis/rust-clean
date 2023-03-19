use std::sync::Arc;

use crate::{
    application::{
        context::{
            interfaces::{ContextRepository, ContextService},
            service::ContextServiceImpl,
        },
        todos::{
            interfaces::{TodoRepository, TodoService},
            service::TodoServiceImpl,
        },
    },
    infrastructure::{
        context::repository::ContextDieselRepository, postgresql::db_pool,
        todos::repository::TodoDieselRepository,
    },
};

pub struct ServiceProvider {
    pub todo_service: Arc<dyn TodoService>,
    pub context_service: Arc<dyn ContextService>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        let pool = db_pool();

        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(Arc::new(pool.clone())));
        let todo_service = Arc::new(TodoServiceImpl {
            repository: todo_repository,
        });

        let context_repository: Arc<dyn ContextRepository> =
            Arc::new(ContextDieselRepository::new(Arc::new(pool.clone())));

        let context_service = Arc::new(ContextServiceImpl {
            repository: context_repository,
        });

        ServiceProvider {
            todo_service,
            context_service,
        }
    }
}

impl Default for ServiceProvider {
    fn default() -> Self {
        Self::new()
    }
}
