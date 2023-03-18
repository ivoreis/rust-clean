use crate::domain::context::entities::ServiceContext;

pub trait ContextService {
    fn get_service_context(&self) -> ServiceContext;
    fn update(&self, service_context: ServiceContext) -> ServiceContext;
    fn is_maintenance_active(&self) -> bool;
}
