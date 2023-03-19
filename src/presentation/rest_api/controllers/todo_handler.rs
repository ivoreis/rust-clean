use actix_web::{web, HttpResponse, Result};

use crate::{
    application::{
        common::{error::ApiError, result::ResultPaging},
        todos::interfaces::{TodoQueryParams, TodoService},
    },
    presentation::dtos::todos::{CreateTodoDTO, TodoDTO},
};

pub async fn create_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    post_data: web::Json<CreateTodoDTO>,
) -> Result<web::Json<TodoDTO>, ApiError> {
    let todo = todo_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(todo.into()))
}

pub async fn list_todos_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Query<TodoQueryParams>,
) -> Result<web::Json<ResultPaging<TodoDTO>>, ApiError> {
    let selection = todo_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Path<i32>,
) -> Result<web::Json<TodoDTO>, ApiError> {
    let todo = todo_service.get(params.into_inner()).await?;
    Ok(web::Json(todo.into()))
}

pub async fn delete_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    todo_service.delete(params.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
