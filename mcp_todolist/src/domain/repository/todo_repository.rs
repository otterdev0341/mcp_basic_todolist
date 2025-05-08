use anyhow::Result;

use crate::domain::dto::todo_dto::{ReqCreateTodoDto, ReqUpdateTodoDto, ResEntryTodoDto};

#[async_trait::async_trait]
#[mockall::automock]
pub trait TodoOperationRepository {
    async fn create_task(&self, dto: ReqCreateTodoDto) -> Result<i32>;
    async fn update_task(&self, task_id: i32, dto: ReqUpdateTodoDto) -> Result<()>;
    async fn get_by_id(&self, task_id: i32) -> Result<ResEntryTodoDto>;
    async fn get_all(&self) -> Result<Vec<ResEntryTodoDto>>;
    async fn delete_task(&self, task_id: i32) -> Result<()>;
}


#[async_trait::async_trait]
#[mockall::automock]
pub trait TodoUtilityRepository {
    async fn count_all_task(&self) -> Result<i32>;
    async fn count_done_task(&self) -> Result<i32>;
    async fn count_undone_task(&self) -> Result<i32>;
}