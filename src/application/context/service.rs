use diesel::{insert_into, prelude::*, result::Error, update};
use log::info;
use std::sync::Arc;

use crate::{
    application::context::interfaces::ContextService,
    domain::context::entities::ServiceContext,
    infrastructure::{context::service_context::ServiceContextDiesel, postgresql::DBConn},
};

#[derive(Clone)]
pub struct ContextServiceImpl {
    pub pool: Arc<DBConn>,
}

impl ContextServiceImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        ContextServiceImpl { pool: db }
    }

    fn get_service_context(&self) -> ServiceContext {
        use crate::infrastructure::schema::service_contexts::dsl::{id, service_contexts};
        let mut conn = self.pool.get().unwrap();
        let result: Result<ServiceContextDiesel, Error> = service_contexts
            .filter(id.eq(1))
            .first::<ServiceContextDiesel>(&mut conn);

        if result.is_err() {
            info!("Service context does not exist, creating a service context...");
            return self.create_service_context();
        }

        result.unwrap().into()
    }

    fn create_service_context(&self) -> ServiceContext {
        use crate::infrastructure::schema::service_contexts::dsl::service_contexts;
        let mut conn = self.pool.get().unwrap();
        let result: Result<ServiceContextDiesel, Error> = insert_into(service_contexts)
            .values(ServiceContextDiesel {
                id: 1,
                maintenance: false,
            })
            .get_result(&mut conn);

        if result.is_err() {
            panic!("Could not create service context");
        }
        result.unwrap().into()
    }
}

impl ContextService for ContextServiceImpl {
    fn get_service_context(&self) -> ServiceContext {
        self.get_service_context()
    }

    fn update(&self, service_context: ServiceContext) -> ServiceContext {
        let service_context_diesel: ServiceContextDiesel =
            ServiceContextDiesel::from(service_context);
        let mut conn = self.pool.get().unwrap();
        use crate::infrastructure::schema::service_contexts::dsl::{id, service_contexts};

        let result: Result<ServiceContextDiesel, Error> = update(service_contexts)
            .filter(id.eq(service_context_diesel.id))
            .set(service_context_diesel)
            .get_result(&mut conn);

        if result.is_err() {
            panic!("Could not update service context");
        }
        result.unwrap().into()
    }

    fn is_maintenance_active(&self) -> bool {
        self.get_service_context().maintenance
    }
}
