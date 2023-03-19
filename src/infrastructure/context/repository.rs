use std::sync::Arc;

use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::{
    application::{common::result::RepositoryResult, context::interfaces::ContextRepository},
    domain::context::entities::ServiceContext,
    infrastructure::{
        context::service_context::ServiceContextDiesel, error::DieselRepositoryError,
        postgresql::DBConn,
    },
};

pub struct ContextDieselRepository {
    pub pool: Arc<DBConn>,
}

impl ContextDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        ContextDieselRepository { pool: db }
    }
}

#[async_trait]
impl ContextRepository for ContextDieselRepository {
    async fn get_service_context(&self) -> RepositoryResult<ServiceContext> {
        use crate::infrastructure::schema::service_contexts::dsl::{id, service_contexts};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            service_contexts
                .filter(id.eq(1))
                .first::<ServiceContextDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())
        .map(|v| -> ServiceContext { v.into() })
    }

    async fn update(&self, service_context: ServiceContext) -> RepositoryResult<ServiceContext> {
        use crate::infrastructure::schema::service_contexts::dsl::{id, service_contexts};
        let mut conn = self.pool.get().unwrap();
        let service_context_diesel: ServiceContextDiesel = service_context.into();
        let result: ServiceContextDiesel = run(move || {
            diesel::update(service_contexts.filter(id.eq(service_context_diesel.id)))
                .set(service_context_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn create_service_context(&self) -> RepositoryResult<ServiceContext> {
        use crate::infrastructure::schema::service_contexts::dsl::service_contexts;
        let mut conn = self.pool.get().unwrap();
        let result: ServiceContextDiesel = run(move || {
            diesel::insert_into(service_contexts)
                .values(ServiceContextDiesel::default())
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }
}
