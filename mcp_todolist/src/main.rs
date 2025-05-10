use std::sync::Arc;

use anyhow::Result;
use mcp_todolist::{
    api_docs::init_open_api::init_openapi, application::usecase::todo_usecase::TodolistUseCase, configuration::config, infrastructure::{
        faring::cors::CORS, http_handler::init_handler::init_controller_setup, mcp_handler::handler::MCPHandler, sqlite::{db_connection::sqlite_con::conn, repository_impl::todolist::TodoListSqliteRepository}
    }
};

use rmcp::{transport::stdio, ServiceExt};
use tokio::signal;
use tracing_subscriber::EnvFilter;
use utoipa_swagger_ui::SwaggerUi;



#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Start the application");

    let db_pool = Arc::new(conn(&config.database_url)?);
    let todo_repo = TodoListSqliteRepository::new(Arc::clone(&db_pool));
    let todo_use_case = Arc::new(TodolistUseCase::new(Arc::new(todo_repo)));

    // MCP service future
    let mcp_service = MCPHandler::new(Arc::clone(&todo_use_case)).serve(stdio());

    // Rocket future
    let rocket = rocket::build()
        .attach(CORS)
        .manage(Arc::clone(&todo_use_case))
        .attach(init_controller_setup())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-doc/openapi.json", init_openapi()),
        )
        .launch();

    // Ctrl+C handler
    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        tracing::info!("Ctrl+C received. Shutting down...");
    };

    // Run all concurrently, exit when any of them ends
    tokio::select! {
        _ = rocket => {
            tracing::info!("Rocket server exited");
        }
        _ = mcp_service => {
            tracing::info!("MCP service exited");
        }
        _ = shutdown_signal => {
            tracing::info!("Shutdown signal received");
        }
    }

    Ok(())
}
