use actix_web::web;
use crate::api::handlers::users;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::get().to(users))
    );
}