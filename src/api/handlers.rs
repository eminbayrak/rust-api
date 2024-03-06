use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn users() -> impl Responder {
    let users = vec![
        json!({"id": 1, "name": "John Doe"}),
        json!({"id": 2, "name": "Jane Doe"}),
        json!({"id": 3, "name": "Joe Bloggs"}),
        json!({"id": 4, "name": "Janet Bloggs"}),
    ];
    HttpResponse::Ok().json(json!({"data": users, "count": users.len()}))
}