mod features;

use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
};
use sqlx::{migrate, postgres::PgPoolOptions};

async fn root_route() -> impl IntoResponse {
    Redirect::to("/client")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new().connect(&db_url).await.unwrap();

    migrate!("./migrations").run(&db).await.unwrap();

    let app = Router::new()
        .nest("/status", features::status::router())
        .nest("/authorization", features::authorization::router())
        .nest("/client", features::client::router())
        .merge(Router::new().route("/", get(root_route)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
