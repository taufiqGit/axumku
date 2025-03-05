use axum::{routing::get, Router, Json, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions, FromRow};
// use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct LogoClub {
    id: i32,
    short_link: String,
    original_link: String,
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
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Server running at http://{}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Route handler: Root
async fn root() -> &'static str {
    "Welcome to Axum with PostgreSQL!"
}

//Route handler: Fetch users from database
// async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
//     let users = sqlx::query!("select id from LogoClub")
//         .fetch_all(&pool)
//         .await
//         .expect("Failed to fetch users");

//     Json(users)
// }


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Resp {
    jum: i32
}
async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, LogoClub>("SELECT id, short_link, original_link FROM links")
        .fetch_all(&pool)
        .await 
    {
        Ok(data) => Json(data).into_response(),
        Err(_) => Json(Vec::<LogoClub>::new()).into_response()
    }
}
