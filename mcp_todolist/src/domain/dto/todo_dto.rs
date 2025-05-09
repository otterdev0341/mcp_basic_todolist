use rmcp::schemars;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::domain::entities::todo_entity::{ NewTodoEntity, UpdateTodoEntity};


#[derive(Deserialize,Serialize, Debug, Clone, schemars::JsonSchema, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateTodoDto{
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "description cannot be empty"))]
    pub description: String,
    pub is_done: bool
}

impl From<ReqCreateTodoDto> for NewTodoEntity {
    fn from(dto: ReqCreateTodoDto) -> Self {
        NewTodoEntity { 
            title: dto.title, 
            description: dto.description, 
            is_done: dto.is_done
        }
    }
}


#[derive(Deserialize,Serialize, Debug, Clone, schemars::JsonSchema, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateTodoDto{
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_done: Option<bool>
}

impl From<ReqUpdateTodoDto> for UpdateTodoEntity {
    fn from(dto: ReqUpdateTodoDto) -> Self {
        UpdateTodoEntity { 
            title: dto.title, 
            description: dto.description, 
            is_done: dto.is_done 
        }
    }
}

#[derive(Deserialize,Serialize, Debug, Clone, schemars::JsonSchema, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResEntryTodoDto{
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: String,
    pub updated_at: String
}


#[derive(Deserialize,Serialize, Debug, Clone, schemars::JsonSchema, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetTaskById {
    pub id: u32
}