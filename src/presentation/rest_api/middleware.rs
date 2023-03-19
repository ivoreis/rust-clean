use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::info;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::application::context::interfaces::ContextService;

pub struct ServiceContextMaintenanceCheck;

impl<S: 'static, B> Transform<S, ServiceRequest> for ServiceContextMaintenanceCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ServiceContextMaintenanceCheckMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServiceContextMaintenanceCheckMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct ServiceContextMaintenanceCheckMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ServiceContextMaintenanceCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let service_context_service = req.app_data::<web::Data<dyn ContextService>>().unwrap();
            let is_active = service_context_service
                .is_maintenance_active()
                .await
                .unwrap();

            if is_active && !req.path().starts_with("/context") {
                info!("Service is in maintenance mode");
                let (request, _pl) = req.into_parts();
                let response = HttpResponse::ServiceUnavailable()
                    .finish()
                    .map_into_right_body();
                return Ok(ServiceResponse::new(request, response));
            }

            let res = svc.call(req).await;
            res.map(ServiceResponse::map_into_left_body)
        })
    }
}
