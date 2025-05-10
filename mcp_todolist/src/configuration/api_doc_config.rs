use utoipa::OpenApi;

use crate::domain::dto::todo_dto::{ReqCreateTodoDto, ResEntryTodoDto};


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Todolist Management API",
        version = "0.1.0",
        description = "API for Entry Project"
    ),
    servers(
        (url = "http://127.0.0.1:8000/v1", description = "Local Development Server"),
        
    ),
)]
pub struct ApiDoc;