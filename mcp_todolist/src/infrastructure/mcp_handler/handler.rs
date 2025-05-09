

use std::sync::Arc;
use rmcp::{model::{AnnotateAble, CallToolResult, Content, RawResource, Resource}, tool, Error as McpError};
use crate::{application::usecase::todo_usecase::TodolistUseCase, domain::dto::todo_dto::{ReqCreateTodoDto, ResEntryTodoDto}};

#[allow(dead_code)]
#[derive(Clone)]
pub struct MCPHandler {
    todo_use_case: Arc<TodolistUseCase>
}


#[tool(tool_box)]
impl MCPHandler {
    
    pub fn new(use_case: Arc<TodolistUseCase>) -> Self {
        Self { todo_use_case: use_case }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "use this to create task in to database")]
    pub async fn create_task(
        &self,
        #[tool(aggr)] dto: ReqCreateTodoDto
    ) -> Result<CallToolResult, McpError> {
        match self.todo_use_case.create_task(dto).await {
            Ok(data) => Ok(CallToolResult::success(vec![Content::text(format!("Task create succesfull id: {}", data))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }
}