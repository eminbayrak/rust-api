use actix_web::web;
use crate::api::handlers::get_user_handler::users;
use crate::api::handlers::add_user_handler::add_user;
use crate::api::handlers::node::node;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(users))
            .route("/adduser", web::post().to(add_user))
            .route("/node", web::get().to(node))
    );
}