use actix_web::web;

use crate::{
    application::{common::error::ApiError, context::interfaces::ContextService},
    presentation::context::ServiceContextDTO,
};

pub async fn update_context_handler(
    service_context_service: web::Data<dyn ContextService>,
    post_data: web::Json<ServiceContextDTO>,
) -> Result<web::Json<ServiceContextDTO>, ApiError> {
    let service_context = service_context_service.update(post_data.into_inner().into());
    Ok(web::Json(service_context.into()))
}

pub async fn get_contex_handler(
    service_context_service: web::Data<dyn ContextService>,
) -> Result<web::Json<ServiceContextDTO>, ApiError> {
    let service_context = service_context_service.get_service_context();
    Ok(web::Json(service_context.into()))
}
