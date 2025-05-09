use std::sync::Arc;

use anyhow::Result;
use mcp_todolist::{application::usecase::todo_usecase::TodolistUseCase, configuration::config, infrastructure::{mcp_handler::handler::MCPHandler, sqlite::{db_connection::sqlite_con::conn, repository_impl::todolist::TodoListSqliteRepository}}};
use rmcp::{ transport::stdio, ServiceExt};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    
    
    let config = config::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Start the application");

    let db_poll = conn(&config.database_url)?;
    let db_pool_arc = Arc::new(db_poll);
    
    let todo_use_case = {
        let todo_repo = TodoListSqliteRepository::new(Arc::clone(&db_pool_arc));
        TodolistUseCase::new(Arc::new(todo_repo))
    };

    let service = MCPHandler::new(
        Arc::new(todo_use_case)
    ).serve(stdio())
    .await
    .inspect_err(|e|{
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    

    Ok(())
}
