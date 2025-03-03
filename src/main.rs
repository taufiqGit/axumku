use axum::{routing::get, Router, Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Define routes
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users))
        .with_state(pool); // Pass database pool as state

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Route handler: Root
async fn root() -> &'static str {
    "Welcome to Axum with PostgreSQL!"
}

// Route handler: Fetch users from database
async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name FROM master.GPRODKMDB")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users");

    Json(users)
}
