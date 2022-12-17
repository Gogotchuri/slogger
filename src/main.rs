use std::sync::Arc;

use app::run;
use logger_service::{interfaces::Service, repository, service};

pub mod app;
pub mod logger_service;
pub mod server;

fn main() {
    let service = initialize_service();
    run(Arc::new(service)).unwrap();
}

fn initialize_service() -> impl Service + Send + Sync {
    let repository = repository::new_in_memory();
    service::new(repository)
}
