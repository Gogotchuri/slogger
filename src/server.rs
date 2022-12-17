use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::logger_service::{interfaces, models::Message};

#[get("/")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    let messages = state.service.get_messages_for_user(1).unwrap();
    let body = serde_json::json!({
            "message": messages[0].message.clone(),
            "count": messages.len(),
    });
    HttpResponse::Ok().body(body.to_string())
}

#[post("/log")]
async fn log(state: web::Data<AppState>, message: web::Json<Message>) -> impl Responder {
    //Borrow service mutably
    state.service.log(message.0).unwrap();
    let messages = state.service.get_messages_for_user(1).unwrap();
    let body = serde_json::json!({
            "message": messages[0].message.clone(),
            "count": messages.len(),
    });
    HttpResponse::Ok().body(body.to_string())
}
pub struct AppState {
    service: Arc<dyn interfaces::Service + Send + Sync>,
}

pub async fn run_server(
    service: Arc<dyn interfaces::Service + Send + Sync>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        let data = web::Data::new(AppState {
            service: service.clone(),
        });
        App::new().app_data(data).service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
