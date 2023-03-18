use serde::{Deserialize, Serialize};

use crate::domain::context::entities::ServiceContext;

#[derive(Deserialize, Serialize)]
pub struct ServiceContextDTO {
    pub id: i32,
    pub maintenance: bool,
}

impl From<ServiceContext> for ServiceContextDTO {
    fn from(service_context: ServiceContext) -> Self {
        ServiceContextDTO {
            id: service_context.id,
            maintenance: service_context.maintenance,
        }
    }
}

impl From<ServiceContextDTO> for ServiceContext {
    fn from(service_context_dto: ServiceContextDTO) -> Self {
        ServiceContext {
            id: service_context_dto.id,
            maintenance: service_context_dto.maintenance,
        }
    }
}
