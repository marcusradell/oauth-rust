mod features;

use std::sync::Arc;

use axum::Router;
use sqlx::{PgPool, migrate, postgres::PgPoolOptions};

#[derive(Clone)]
struct AppState {
    db: PgPool,
    jwt_secret: String,
    issuer: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new().connect(&db_url).await.unwrap();

    migrate!("./migrations").run(&db).await.unwrap();

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());

    let state = Arc::new(AppState {
        db,
        jwt_secret,
        issuer: "http://localhost:3000".to_string(),
    });

    let app = Router::new()
        .nest("/status", features::status::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
