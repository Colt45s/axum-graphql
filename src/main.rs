mod entities;
mod handlers;
mod schema;

use std::{env, sync::Arc, time::Duration};

use async_graphql::{dataloader::DataLoader, EmptyMutation, EmptySubscription};
use axum::{
    routing::{get, post},
    Router,
};
use schema::Schema;
use sea_orm::{ConnectOptions, Database};

use crate::{
    handlers::{graphql_handler, graphql_playground},
    schema::Query,
};

use crate::schema::post::loader::PostLoader;

pub struct AppState {
    pub schema: Arc<Schema>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut option = ConnectOptions::new(database_url);
    option
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(3))
        .sqlx_logging(true);

    let conn = Database::connect(option)
        .await
        .expect("Failed to connect to database");

    let post_loader = DataLoader::new(PostLoader::new(conn.clone()), tokio::spawn);

    let schema = Arc::new(
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(conn)
            .data(post_loader)
            .limit_complexity(5000)
            // .limit_depth(5)
            .finish(),
    );

    let state = Arc::new(AppState { schema });
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphql_playground))
        .with_state(state);

    println!("Playground: http://localhost:4000/graphiql");

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
