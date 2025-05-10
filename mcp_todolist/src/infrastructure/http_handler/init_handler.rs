use rocket::fairing::AdHoc;

use super::http_handler::todolist_routes;



pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initial Controller", |rocket| async {
        rocket
            .mount("/v1", todolist_routes())
    })
}