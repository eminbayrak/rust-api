use actix_web::{HttpResponse, Responder};
use crate::api::models::{User, UsersResponse};

pub async fn users() -> impl Responder {
    let users = vec![
        User { id: 1, name: "John Doe".into() },
        User { id: 2, name: "Jane Doe".into() },
        User { id: 3, name: "Joe Bloggs".into() },
        User { id: 4, name: "Janet Bloggs".into() },
    ];
    let response = UsersResponse { data: users.clone(), count: users.len() };
    HttpResponse::Ok().json(response)
}