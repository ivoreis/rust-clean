use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{common::error::CommonError, context::interfaces::ContextService},
    domain::context::entities::ServiceContext,
};

use super::interfaces::ContextRepository;

#[derive(Clone)]
pub struct ContextServiceImpl {
    pub repository: Arc<dyn ContextRepository>,
}

impl ContextServiceImpl {
    pub fn new(repository: Arc<dyn ContextRepository>) -> Self {
        ContextServiceImpl { repository }
    }
}

#[async_trait]
impl ContextService for ContextServiceImpl {
    async fn get_service_context(&self) -> Result<ServiceContext, CommonError> {
        self.repository.get_service_context().await.or(self
            .repository
            .create_service_context()
            .await
            .map_err(|e| -> CommonError { e.into() }))
    }

    async fn update(&self, service_context: ServiceContext) -> Result<ServiceContext, CommonError> {
        self.repository
            .update(service_context)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn is_maintenance_active(&self) -> Result<bool, CommonError> {
        let service_context = self.get_service_context().await?;
        Ok(service_context.maintenance)
    }
}
