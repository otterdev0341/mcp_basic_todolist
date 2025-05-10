use std::sync::Arc;
use rocket::{delete, get, put, routes, Route};
use rocket::{http::Status, post, serde::json::Json, State};
use crate::domain::dto::todo_dto::ReqUpdateTodoDto;
use crate::infrastructure::http_handler::response_type::Response;
use crate::{application::usecase::todo_usecase::TodolistUseCase, domain::dto::todo_dto::{ReqCreateTodoDto, ResEntryTodoDto}};

use super::response_type::{ErrorResponse, SuccessResponse};



pub fn todolist_routes() -> Vec<Route> {
    routes![
        create_todo,
        update_todo,
        get_by_id,
        get_all,
        delete_todo,
        count_all_task,
        count_done_task,
        count_undone_task
    ]
}


/// Create a new todo entry.
///
/// This endpoint allows the client to create a new todo item by providing a title and description.
/// Upon successful creation, the server returns the full details of the newly created task.
///
/// # Request Body
/// - `title`: Title of the task (String)
/// - `description`: Detailed information about the task (String)
///
/// # Responses
/// - `200 OK`: Task created successfully, returns a `ResEntryTodoDto` object.
/// - `400 Bad Request`: Task creation failed due to invalid input or internal error.
#[utoipa::path(
    post,
    path = "/todo",
    request_body = ReqCreateTodoDto,
    responses(
        (status = 200, description = "Todo created successfully", body = ResEntryTodoDto),
        (status = 400, description = "Failed to create todo. Input validation failed or internal error occurred")
    )
)]
#[post("/todo", data = "<todo_data>")]
pub async fn create_todo(
    todo_data: Json<ReqCreateTodoDto>,
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> {
    match state.create_task(todo_data.into_inner()).await {
        Ok(data) => Ok(SuccessResponse((Status::Ok, format!("Task create succesfull {:?}",data)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Failed to create task please try again"))))
    }
}





/// Update an existing todo entry.
///
/// This endpoint allows the client to update an existing todo item. Only the fields
/// provided in the request body will be updated (partial updates supported).
///
/// # Request Body
/// - `id`: ID of the todo item to update (i32)
/// - `title`: (Optional) New title for the task
/// - `description`: (Optional) New description for the task
/// - `is_done`: (Optional) Boolean to mark task as done or not
///
/// # Responses
/// - `200 OK`: Task updated successfully, returns the updated `ResEntryTodoDto`
/// - `400 Bad Request`: Update failed due to missing ID, invalid input, or task not found
#[utoipa::path(
    put,
    path = "/todo",
    request_body = ReqUpdateTodoDto,
    responses(
        (status = 200, description = "Todo updated successfully", body = ResEntryTodoDto),
        (status = 400, description = "Failed to update todo. Invalid input or task not found")
    )
)]
#[put("/todo", data = "<todo_data>")]
pub async fn update_todo(
    todo_data: Json<ReqUpdateTodoDto>,
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> {
    let json_val = todo_data.clone();
    let extract_id = json_val.into_inner().id;
    match state.update_task(extract_id, todo_data.into_inner()).await {
        Ok(data) => Ok(SuccessResponse((Status::Ok, format!("Task Update Successfull {:?}", data)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Failed to update the task"))))
    }
}





/// Retrieve a todo entry by ID.
///
/// This endpoint allows the client to fetch a specific todo item by its unique identifier.
/// The server returns the full task details if found.
///
/// # Path Parameters
/// - `todo_id`: ID of the todo item to retrieve (as a string, parsed to i32)
///
/// # Responses
/// - `200 OK`: Task retrieved successfully, returns a `ResEntryTodoDto` object
/// - `400 Bad Request`: Failed to retrieve task, either due to invalid ID format or item not found
#[utoipa::path(
    get,
    path = "/todo/{todo_id}",
    params(
        ("todo_id" = i32, Path, description = "Unique identifier of the todo task")
    ),
    responses(
        (status = 200, description = "Todo retrieved successfully", body = ResEntryTodoDto),
        (status = 400, description = "Failed to retrieve todo. Invalid ID or item not found")
    )
)]
#[get("/todo/<todo_id>")]
pub async fn get_by_id(
    todo_id: String,
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> {
    let id: i32 = todo_id.parse().unwrap();
    match state.get_by_id(id).await {
        Ok(data) => Ok(SuccessResponse((Status::Ok, format!("{:?}",data)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Fail to get todo by id : {:?}",todo_id))))
    }
}




/// Retrieve all todo entries.
///
/// This endpoint returns a list of all existing todo tasks. Each item contains the full details
/// of the task including ID, title, description, completion status, and timestamps.
///
/// # Responses
/// - `200 OK`: Returns a list of all todo tasks as `Vec<ResEntryTodoDto>`
/// - `400 Bad Request`: Failed to retrieve tasks due to an internal error
#[utoipa::path(
    get,
    path = "/todo",
    responses(
        (status = 200, description = "All todos retrieved successfully", body = [ResEntryTodoDto]),
        (status = 400, description = "Failed to retrieve todos. Internal error occurred")
    )
)]
#[get("/todo")]
pub async fn get_all(
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> {
    match state.get_all().await {
        Ok(data) => Ok(SuccessResponse((Status::Ok, format!("{:?}", data)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Failed to get all todo"))))
    }
}








/// Delete a todo entry by ID.
///
/// This endpoint deletes a specific todo task using its unique ID. If the task exists,
/// it will be permanently removed from the database.
///
/// # Path Parameters
/// - `todo_id`: ID of the todo task to delete (as a string, parsed to i32)
///
/// # Responses
/// - `200 OK`: Task deleted successfully, returns the deleted task ID or confirmation
/// - `400 Bad Request`: Deletion failed due to invalid ID or task not found
#[utoipa::path(
    delete,
    path = "/todo/{todo_id}",
    params(
        ("todo_id" = i32, Path, description = "Unique identifier of the todo task to delete")
    ),
    responses(
        (status = 200, description = "Todo deleted successfully"),
        (status = 400, description = "Failed to delete todo. Invalid ID or task not found")
    )
)]
#[delete("/todo/<todo_id>")]
pub async fn delete_todo(
    todo_id: String,
    state: &State<Arc<TodolistUseCase>>
) -> Response<String>
{
    let id: i32 = todo_id.parse().unwrap();
    match state.delete_task(id).await {
        Ok(data) => Ok(SuccessResponse((Status::Ok, format!("Task id : {:?} has deleted", data)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Fail to delte task id: {:?}", id))))
    }
}





/// Count all todo tasks.
///
/// This endpoint returns the total number of todo items stored in the system.
///
/// # Responses
/// - `200 OK`: Successfully counted tasks, returns the total count as a number wrapped in a string message
/// - `400 Bad Request`: Failed to count tasks due to an internal error
#[utoipa::path(
    get,
    path = "/todo/all",
    responses(
        (status = 200, description = "Successfully retrieved the total number of todo items"),
        (status = 400, description = "Failed to retrieve todo count due to an internal error")
    )
)]
#[get("/todo/all")]
pub async fn count_all_task(
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> 
{
    match state.count_all_task().await {
        Ok(items) => Ok(SuccessResponse((Status::Ok, format!("all todo have {:?} items",items)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Fail to get all count"))))
    }
}







/// Count completed todo tasks.
///
/// This endpoint returns the total number of todo items that have been marked as done.
///
/// # Responses
/// - `200 OK`: Successfully counted completed tasks, returns the total number in a message string
/// - `400 Bad Request`: Failed to retrieve the count due to an internal error
#[utoipa::path(
    get,
    path = "/todo/done",
    responses(
        (status = 200, description = "Successfully retrieved the total number of completed todo items"),
        (status = 400, description = "Failed to retrieve completed todo count due to an internal error")
    )
)]
#[get("/todo/done")]
pub async fn count_done_task(
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> {
    match state.count_done_task().await {
        Ok(items) => Ok(SuccessResponse((Status::Ok, format!("all todo have {:?} task that mark as done",items)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Fail to count done task"))))
    }
}








/// Count incomplete (undone) todo tasks.
///
/// This endpoint returns the total number of todo items that have **not** been marked as done.
///
/// # Responses
/// - `200 OK`: Successfully counted undone tasks, returns the total number in a message string
/// - `400 Bad Request`: Failed to retrieve the count due to an internal error
#[utoipa::path(
    get,
    path = "/todo/undone",
    responses(
        (status = 200, description = "Successfully retrieved the total number of undone todo items"),
        (status = 400, description = "Failed to retrieve undone todo count due to an internal error")
    )
)]
#[get("/todo/undone")]
pub async fn count_undone_task(
    state: &State<Arc<TodolistUseCase>>
) -> Response<String> {
    match state.count_undone_task().await {
        Ok(items) => Ok(SuccessResponse((Status::Ok, format!("all todo have {:?} task that mark as undone",items)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, format!("Fail to count undone task"))))
    }
}