use std::sync::Arc;

use crate::domain::repository::todo_repository::{TodoOperationRepository, TodoUtilityRepository};

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
}