use std::sync::Arc;
use anyhow::{Result, Context};
use crate::{domain::{dto::todo_dto::{ReqCreateTodoDto, ReqUpdateTodoDto, ResEntryTodoDto}, entities::todo_entity::{EntryTodoEntity, NewTodoEntity, UpdateTodoEntity}, repository::todo_repository::{TodoOperationRepository, TodoUtilityRepository}, schema::schema::todolist}, infrastructure::sqlite::db_connection::sqlite_con::SqlitePoolSquad};
use diesel::{RunQueryDsl, dsl::insert_into, update, QueryDsl, ExpressionMethods};

#[derive(Clone)]
pub struct TodoListSqliteRepository {
    db_pool: Arc<SqlitePoolSquad>
}

impl TodoListSqliteRepository {
    pub fn new(db_pool: Arc<SqlitePoolSquad>) -> Self {
        Self { 
            db_pool 
        }
    }
}


#[async_trait::async_trait]
impl TodoOperationRepository for TodoListSqliteRepository {
    
    async fn create_task(&self, dto: ReqCreateTodoDto) -> Result<ResEntryTodoDto> {
        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let entity: NewTodoEntity = dto.into();

        let inserted: EntryTodoEntity = insert_into(todolist::table)
            .values(entity)
            .returning(todolist::all_columns)
            .get_result(conn)
            .context("Failed to insert new todo into database")?;

        Ok(inserted.into())
    }

    async fn update_task(&self, task_id: i32, dto: ReqUpdateTodoDto) -> Result<ResEntryTodoDto> {
        
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let the_entity: UpdateTodoEntity = dto.into();

        let updated_rows = update(todolist.filter(id.eq(task_id)))
        .set(the_entity)
        .execute(conn)
        .context("Failed to update todo item")?;
        
        if updated_rows == 0 {
            anyhow::bail!("No todo item found with id {}", task_id);
        }

        let result: EntryTodoEntity = todolist
            .filter(id.eq(task_id))
            .first(conn)
            .context(format!("Failed to get Data"))?;

        Ok(result.into())
    }
    async fn get_by_id(&self, task_id: i32) -> Result<ResEntryTodoDto> {
        
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let todo: EntryTodoEntity = todolist
            .filter(id.eq(task_id))
            .first(conn)
            .context(format!("Todo with id {} not found", task_id))?;

        Ok(todo.into())
    }


    async fn get_all(&self) -> Result<Vec<ResEntryTodoDto>> {
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let todos: Vec<EntryTodoEntity> = todolist
            .load(conn)
            .context("Failed to load todo items from the database")?;

        let result: Vec<ResEntryTodoDto> = todos.into_iter().map(|todo| todo.into()).collect();

        Ok(result)
    }
    async fn delete_task(&self, task_id: i32) -> Result<()> {
        
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let deleted_rows = diesel::delete(todolist.filter(id.eq(task_id)))
        .execute(conn)
        .context("Failed to delete todo item")?;

        if deleted_rows == 0 {
            anyhow::bail!("No todo item found with id {}", task_id);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl TodoUtilityRepository for TodoListSqliteRepository {
    async fn count_all_task(&self) -> Result<i32>{
        
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let count: i64 = todolist
        .count() // count all rows
        .get_result(conn)
        .context("Failed to count all todo items in the database")?;

        // Since Diesel returns `i64` for the count, we need to cast it to `i32`
        Ok(count as i32)
    }
    async fn count_done_task(&self) -> Result<i32>{
        
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let count: i64 = todolist
        .filter(is_done.eq(true)) // Filter to count only done tasks
        .count()
        .get_result(conn)
        .context("Failed to count done todo items in the database")?;

        Ok(count as i32)
    }
    async fn count_undone_task(&self) -> Result<i32>{
        use crate::domain::schema::schema::todolist::dsl::*;

        let conn = &mut self
            .db_pool
            .get()
            .context("Failed to get DB connection from pool")?;

        let count: i64 = todolist
            .filter(is_done.eq(false)) // Filter to count only undone tasks
            .count()
            .get_result(conn)
            .context("Failed to count undone todo items in the database")?;

        Ok(count as i32)
    }
}