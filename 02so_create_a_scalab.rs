// 02so_create_a_scalab.rs

use anyhow::Result;
use async_std::task;
use serde::{Deserialize, Serialize};

// Configuration Struct
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database_url: String,
    api_port: u16,
    secret_key: String,
}

// API Error
#[derive(Debug, Serialize, Deserialize)]
enum ApiError {
    DatabaseError(String),
    InvalidRequest(String),
}

// API Response
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    message: String,
}

async fn start_api(config: Config) -> Result<()> {
    // Initialize Database Connection
    let db_url = config.database_url.clone();
    let _db_conn = sqlx::PgPool::connect(&db_url).await?;

    // Start API Server
    let app = actix_web::web::service(
        actix_web::web::resource("/api/health")
            .route(actix_web::web::get().to(health_check)),
    );

    actix_web::HttpServer::new(move || app)
        .bind(format!("127.0.0.1:{}", config.api_port))?
        .run()
        .await?;

    Ok(())
}

async fn health_check() -> actix_web::HttpResponse {
    let response = ApiResponse {
        message: "API is healthy!".to_string(),
    };

    actix_web::HttpResponse::Ok().json(response)
}

#[async_std::main]
async fn main() -> Result<()> {
    let config = Config {
        database_url: "postgresql://username:password@localhost/db_name".to_string(),
        api_port: 8080,
        secret_key: "secret_key_here".to_string(),
    };

    start_api(config).await?;

    Ok(())
}