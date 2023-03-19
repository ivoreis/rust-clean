use async_trait::async_trait;

use crate::{
    application::common::{error::CommonError, result::RepositoryResult},
    domain::context::entities::ServiceContext,
};

#[async_trait]
pub trait ContextService: Send + Sync {
    async fn get_service_context(&self) -> Result<ServiceContext, CommonError>;
    async fn update(&self, service_context: ServiceContext) -> Result<ServiceContext, CommonError>;
    async fn is_maintenance_active(&self) -> Result<bool, CommonError>;
}

#[async_trait]
pub trait ContextRepository: Send + Sync {
    async fn get_service_context(&self) -> RepositoryResult<ServiceContext>;
    async fn update(&self, service_context: ServiceContext) -> RepositoryResult<ServiceContext>;
    async fn create_service_context(&self) -> RepositoryResult<ServiceContext>;
}
