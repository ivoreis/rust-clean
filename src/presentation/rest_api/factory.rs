use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::Logger,
    web, App, Error,
};

use crate::presentation::{
    provider::ServiceProvider,
    rest_api::{
        controllers::{
            context_handler::{get_contex_handler, update_context_handler},
            todo_handler::{
                create_todo_handler, delete_todo_handler, get_todo_handler, list_todos_handler,
            },
        },
        middleware::ServiceContextMaintenanceCheck,
    },
};

fn todos() -> actix_web::Scope {
    web::scope("/todos")
        .route("", web::post().to(create_todo_handler))
        .route("", web::get().to(list_todos_handler))
        .route("/{id}", web::get().to(get_todo_handler))
        .route("/{id}", web::delete().to(delete_todo_handler))
}

fn context() -> actix_web::Scope {
    web::scope("/context")
        .route("", web::get().to(get_contex_handler))
        .route("", web::put().to(update_context_handler))
}

pub fn create() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let provider = ServiceProvider::new();

    App::new()
        .app_data(web::Data::from(provider.todo_service.clone()))
        .app_data(web::Data::from(provider.context_service.clone()))
        .wrap(Logger::default())
        .wrap(ServiceContextMaintenanceCheck)
        .service(todos())
        .service(context())
}
