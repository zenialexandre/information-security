mod model;
mod schema;
mod handler;
mod route;

use axum::{
    http::{
        header::CONTENT_TYPE,
        Method
    },
    Router
};

use sqlx::{
    mysql::{
        MySqlPool,
        MySqlPoolOptions
    },
    MySql,
    Pool
};

use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;

pub struct AppState {
    pub database: MySqlPool
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must exists");
    let my_sql_pool: Pool<MySql> = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(my_sql_pool) => {
            println!("Connection to the database is successful!");
            my_sql_pool
        },
        Err(error) => {
            println!("Failed to connect to the database: {:?}", error);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);
    let router: Router = route::create_router(Arc::new(AppState { database: my_sql_pool.clone() })).layer(cors);

    println!("Server started successfully at 0.0.0.0:8080");

    let tcp_listener: TcpListener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(
        tcp_listener,
        router.into_make_service()
    ).await.unwrap();
}
