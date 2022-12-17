use std::sync::Arc;

use crate::{logger_service::interfaces, server};

#[actix_web::main]
pub async fn run(service: Arc<dyn interfaces::Service + Send + Sync>) -> std::io::Result<()> {
    server::run_server(service).await
}
