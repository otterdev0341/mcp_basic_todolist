use std::sync::Arc;
use anyhow::{anyhow, Result};

use crate::domain::{dto::todo_dto::{ReqCreateTodoDto, ReqUpdateTodoDto, ResEntryTodoDto}, repository::todo_repository::{TodoOperationRepository, TodoUtilityRepository}};

pub trait TodoRepository: TodoOperationRepository + TodoUtilityRepository {}
impl<T> TodoRepository for T where T: TodoOperationRepository + TodoUtilityRepository {}

#[allow(dead_code)]
pub struct TodolistUseCase {
    
    todo_repo: Arc<dyn TodoRepository + Send + Sync + 'static>,
    
}


impl TodolistUseCase {
    pub fn new(repo: Arc<dyn TodoRepository + Send + Sync + 'static>) -> Self {
        Self{
            todo_repo: repo
        }
    }

    pub async fn create_task(&self, dto: ReqCreateTodoDto) -> Result<i32> {
        let result = self.todo_repo.create_task(dto).await;
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Fail to create")),
        }
    }
    pub async fn update_task(&self, task_id: i32, dto: ReqUpdateTodoDto) -> Result<()> {
        let result = self.todo_repo.update_task(task_id, dto).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("Fail to update")),
        }
    }
    pub async fn get_by_id(&self, task_id: i32) -> Result<ResEntryTodoDto> {
        let result = self.todo_repo.get_by_id(task_id).await;
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Fail to retrive")),
        }
    }
    pub async fn get_all(&self) -> Result<Vec<ResEntryTodoDto>> {
        let result = self.todo_repo.get_all().await;
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Fail to get all tasks")),
        }
    }
    pub async fn delete_task(&self, task_id: i32) -> Result<()> {
        let result = self.todo_repo.delete_task(task_id).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("Fail to delete")),
        }
    }

    pub async fn count_all_task(&self) -> Result<i32> {
        let result = self.todo_repo.count_all_task().await;
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Fail to count all tasks")),
        }
    }
    pub async fn count_done_task(&self) -> Result<i32> {
        let result = self.todo_repo.count_done_task().await;
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Fail to count done task")),
        }
    }
    pub async fn count_undone_task(&self) -> Result<i32> {
        let result = self.todo_repo.count_undone_task().await;
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(anyhow!("Fail to count undone task")),
        }
    }
}
