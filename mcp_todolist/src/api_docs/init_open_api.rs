use utoipa::OpenApi;

use crate::configuration::api_doc_config::ApiDoc;

use super::todolist_api::TodolistApi;



pub fn init_openapi() -> utoipa::openapi::OpenApi {
    let register: Vec<utoipa::openapi::OpenApi> = vec![
        TodolistApi::openapi(),
        ApiDoc::openapi()
    ];

    let mut all_api = register.into_iter();
    let mut merged_api = all_api.next().unwrap();

    for api in all_api {
        merged_api.merge(api);
    }

    merged_api
}