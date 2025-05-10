use utoipa::OpenApi;

use crate::domain::dto::todo_dto::{ReqCreateTodoDto, ReqUpdateTodoDto, ResEntryTodoDto};



#[derive(OpenApi)]
#[openapi(
    security(),
    modifiers(),
    paths(
        crate::infrastructure::http_handler::http_handler::create_todo,
        crate::infrastructure::http_handler::http_handler::update_todo,
        crate::infrastructure::http_handler::http_handler::get_by_id,
        crate::infrastructure::http_handler::http_handler::get_all,
        crate::infrastructure::http_handler::http_handler::delete_todo,
        crate::infrastructure::http_handler::http_handler::count_all_task,
        crate::infrastructure::http_handler::http_handler::count_done_task,
        crate::infrastructure::http_handler::http_handler::count_undone_task,
    ),
    components(
        schemas(
            ResEntryTodoDto,
            ReqCreateTodoDto,
            ReqUpdateTodoDto
        )
    )
)]
pub struct TodolistApi;