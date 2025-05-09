

use std::sync::Arc;
use rmcp::{model::{AnnotateAble, CallToolResult, Content, RawResource, Resource}, tool, Error as McpError};
use crate::{application::usecase::todo_usecase::TodolistUseCase, domain::dto::todo_dto::{ReqCreateTodoDto, GetTaskById, ReqUpdateTodoDto}};

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

    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อสร้างงานใหม่ในระบบ / Use this to create a new task.

        📥 JSON Payload ตัวอย่าง / Example:
        {
        "title": "Buy groceries",
        "description": "Milk, eggs, and bread",
        "is_done": false
        }

        Fields:
        - title (string): ชื่อของงาน / Title of the task
        - description (string): รายละเอียดของงาน / Task description
        - is_done (boolean): งานเสร็จหรือยัง / Whether the task is completed
    "#)]
    pub async fn create_task(
        &self,
        #[tool(aggr)] dto: ReqCreateTodoDto
    ) -> Result<CallToolResult, McpError> {
        match self.todo_use_case.create_task(dto).await {
            Ok(data) => Ok(CallToolResult::success(vec![Content::text(format!("Task create succesfull id: {}", data))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }


    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อดึงรายละเอียดของงานจากระบบตาม ID / Use this to retrieve the details of a task by its ID.

        📥 JSON Payload ตัวอย่าง / Example:
        {
        "id": 1
        }

        Field:
        - id (integer): รหัสของงาน / ID of the task to retrieve

        📤 JSON Response Example:
        {
        "id": 1,
        "title": "Buy groceries",
        "description": "Milk, eggs, and bread",
        "is_done": false,
        "created_at": "2025-05-09T10:45:00Z"
        }

        Response Fields:
        - id (integer): รหัสของงาน / ID of the task
        - title (string): ชื่อของงาน / Title of the task
        - description (string): รายละเอียดของงาน / Task description
        - is_done (boolean): งานเสร็จหรือยัง / Whether the task is completed
        - created_at (string): เวลาที่สร้าง / Timestamp when the task was created
    "#)]
    pub async fn get_by_id(
        &self,
        #[tool(aggr)] dto: GetTaskById
    ) -> Result<CallToolResult, McpError>
    {
        match self.todo_use_case.get_by_id(dto.id as i32).await {
            Ok(data) => {
                if let Ok(convert) = Content::json(data) {
                    Ok(CallToolResult::success(vec![convert]))
                } else {
                    Err(McpError::internal_error("Failed to convert results to JSON".to_string(), None))
                }
            },
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }



    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อดึงข้อมูลงานทั้งหมดในระบบ / Use this command to retrieve all tasks from the system.

        📤 Response ตัวอย่าง / Example Response:
        [
        {
            "id": 1,
            "title": "Buy groceries",
            "description": "Milk, eggs, and bread",
            "is_done": false,
            "created_at": "2024-05-01T12:00:00Z",
            "updated_at": "2024-05-02T14:30:00Z"
        },
        {
            "id": 2,
            "title": "Do laundry",
            "description": "Wash and dry clothes",
            "is_done": true,
            "created_at": "2024-05-01T13:00:00Z",
            "updated_at": "2024-05-01T18:00:00Z"
        }
        ]

        📝 Description:
        - ใช้สำหรับดึงข้อมูลของงานทั้งหมด / Used to retrieve all task entries
        - ข้อมูลที่ได้จะอยู่ในรูปแบบของรายการ (array) ที่ประกอบด้วย `ResEntryTodoDto`
        - สามารถนำไปใช้แสดงใน UI หรือการวิเคราะห์ต่อไปได้
    "#)]
    pub async fn get_all(
        &self
    ) -> Result<CallToolResult, McpError> {
        match self.todo_use_case.get_all().await {
            Ok(inner_data) => {
                if let Ok(convert) = Content::json(inner_data) {
                    Ok(CallToolResult::success(vec![convert]))
                } else {
                    Err(McpError::internal_error("Failed to convert results to Json", None))
                }
            },
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }


    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อลบงานจากระบบโดยระบุรหัสของงาน / Use this command to delete a task from the system by providing its ID.

        📥 JSON Payload ตัวอย่าง / Example Request:
        {
        "id": 1
        }

        🧾 รายละเอียด / Details:
        - ฟิลด์ `id` (integer): รหัสของงานที่ต้องการลบ / The ID of the task to be deleted
        - หากพบงานที่มี `id` ดังกล่าว จะทำการลบทันที / If a task with the given ID is found, it will be deleted
        - ไม่คืนข้อมูลเนื้อหา นอกจากข้อความแจ้งสถานะการลบ / Does not return data, only a status message
        - คำตอบจะอยู่ในรูปแบบ: `"Task delete successful!!!"`

        🛑 หมายเหตุ / Note:
        - หากไม่พบงานที่มี ID ดังกล่าว ระบบจะส่งข้อผิดพลาดกลับ / If no task is found with the given ID, an error will be returned
    "#)]
    pub async fn delete_task(
        &self,
        #[tool(aggr)] dto: GetTaskById
    ) -> Result<CallToolResult, McpError> {
        match self.todo_use_case.delete_task(dto.id as i32).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!("Task delete succesfull!!!"))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }




    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่ออัปเดตข้อมูลของงานที่มีอยู่ โดยระบุรหัสของงานและข้อมูลที่ต้องการเปลี่ยน /  
        Use this command to update an existing task by specifying its ID and the fields to be updated.

        📥 JSON Payload ตัวอย่าง / Example Request:
        {
        "id": 1,
        "title": "Buy groceries and fruits",
        "description": "Milk, eggs, bread, and bananas",
        "is_done": true
        }

        🧾 รายละเอียดฟิลด์ / Field Descriptions:
        - id (integer): รหัสของงานที่ต้องการอัปเดต / The ID of the task to update (required)
        - title (string, optional): ชื่อใหม่ของงาน / New title for the task
        - description (string, optional): รายละเอียดใหม่ของงาน / New description for the task
        - is_done (boolean, optional): สถานะความสำเร็จของงาน / Updated completion status

        📤 ผลลัพธ์ / Response:
        - หากสำเร็จ ระบบจะส่งข้อความ `"Task update successful!!!"` กลับ / 
        On success, returns the message `"Task update successful!!!"`
        - หากไม่พบงาน ระบบจะส่งข้อผิดพลาดกลับ / 
        If the task is not found, an error will be returned
    "#)]
    pub async fn update_task(
        &self,
        #[tool(aggr)] dto: ReqUpdateTodoDto
    ) -> Result<CallToolResult, McpError>
    {
        match self.todo_use_case.update_task(dto.id as i32, dto).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!("Task update succesfull!!!"))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }



    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อนับจำนวนงานทั้งหมดที่มีอยู่ในระบบ /  
        Use this command to count the total number of tasks in the system.

        📤 ผลลัพธ์ / Response:
        - ระบบจะส่งข้อความระบุจำนวนงานทั้งหมด เช่น `"Task have: 10 items"` /  
        Returns a message indicating how many tasks exist, e.g., `"Task have: 10 items"`
    "#)]
    pub async fn count_all_task(
        &self
    ) -> Result<CallToolResult, McpError>
    {
        match self.todo_use_case.count_all_task().await {
            Ok(data) => Ok(CallToolResult::success(vec![Content::text(format!("Task have: {} items", data))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }


    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อนับจำนวนงานที่ทำเสร็จแล้วในระบบ /  
        Use this command to count the number of tasks marked as done in the system.

        📤 ผลลัพธ์ / Response:
        - ระบบจะส่งข้อความระบุจำนวนงานที่เสร็จแล้ว เช่น `"You have 5 tasks, mark as done"` /  
        Returns a message indicating how many tasks have been marked as done, e.g., `"You have 5 tasks, mark as done"`
    "#)]
    pub async fn count_done_task(
        &self
    ) -> Result<CallToolResult, McpError>
    {
        match self.todo_use_case.count_done_task().await {
            Ok(data) => Ok(CallToolResult::success(vec![Content::text(format!("You have {} tasks, mark as done", data))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }



    #[tool(description = r#"
        ใช้คำสั่งนี้เพื่อนับจำนวนงานที่ยังไม่เสร็จในระบบ /  
        Use this command to count the number of tasks that are still marked as not done.

        📤 ผลลัพธ์ / Response:
        - ระบบจะส่งข้อความระบุจำนวนงานที่ยังไม่เสร็จ เช่น `"You have 3 tasks, mark as undone"` /  
        Returns a message indicating how many tasks are not yet completed, e.g., `"You have 3 tasks, mark as undone"`
    "#)]
    pub async fn count_undone_task(
        &self
    ) -> Result<CallToolResult, McpError>
    {
        match self.todo_use_case.count_undone_task().await {
            Ok(data) => Ok(CallToolResult::success(vec![Content::text(format!("You have {} tasks, mark as undone", data))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None))
        }
    }
}