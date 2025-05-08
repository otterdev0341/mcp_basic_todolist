use diesel::prelude::*;
use crate::domain::{dto::todo_dto::ResEntryTodoDto, schema::schema::todolist};



#[derive(Insertable)]
#[diesel(table_name=todolist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTodoEntity {
    pub title: String,
    pub description: String,
    pub is_done: bool
}


#[derive(AsChangeset)]
#[diesel(table_name = todolist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateTodoEntity {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_done: Option<bool>,
}


#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name=todolist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EntryTodoEntity {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}


impl From<EntryTodoEntity> for ResEntryTodoDto {
    fn from(the_entity: EntryTodoEntity) -> Self {
        
        ResEntryTodoDto { 
            id: the_entity.id as u32, 
            title: the_entity.title, 
            description: the_entity.description, 
            is_done: the_entity.is_done, 
            created_at: the_entity.created_at.unwrap_or_else(||"".to_string()), 
            updated_at: the_entity.updated_at.unwrap_or_else(||"".to_string()) 
        }
    }
}