use std::sync::Arc;

use rocket::{get, http::Status, launch, routes, State};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api_docs::init_open_api::init_openapi, application::usecase::todo_usecase::TodolistUseCase, configuration::{api_doc_config::ApiDoc, config}, infrastructure::{http_handler::{http_handler::todolist_routes, init_handler::init_controller_setup}, sqlite::{db_connection::sqlite_con::conn, repository_impl::todolist::TodoListSqliteRepository}}};


#[get("/todos/undone_count")]
async fn count_undone(state: &State<Arc<TodolistUseCase>>) -> Result<String, Status> {
    match state.count_undone_task().await {
        Ok(count) => Ok(format!("You have {} tasks, mark as undone", count)),
        Err(_) => Err(Status::InternalServerError),
    }
}



#[rocket::main]
pub async fn rocket() -> Result<(), rocket::Error> {
    // inject env
    let config = config::load().expect("Failed to load configuration");

    // initial log process to help debug
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    // inject db connection, and Arc for assign accross multithread
    let db_poll = conn(&config.database_url).expect("Failed to connect to database");
    let db_pool_arc = Arc::new(db_poll);

    // inject use case
    let todo_use_case = {
        let todo_repo = TodoListSqliteRepository::new(Arc::clone(&db_pool_arc));
        TodolistUseCase::new(Arc::new(todo_repo))
    };

    // initial web server
    rocket::build()
        .manage(Arc::new(todo_use_case))
        .attach(init_controller_setup())
        .mount("/", 
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("api-doc/openapi.json", 
                init_openapi()
            )
        )   
        .launch()
        .await?;

    Ok(())
}