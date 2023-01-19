mod entities;
mod handlers;
mod schema;

use crate::schema::post::loader::PostLoader;
use crate::{
    handlers::{graphql_handler, graphql_playground},
    schema::{Mutation, Query},
};
use async_graphql::{dataloader::DataLoader, extensions::OpenTelemetry, EmptySubscription};
use axum::{
    routing::{get, post},
    Router,
};
use opentelemetry::{sdk::trace::TracerProvider, trace::TracerProvider as _};
use opentelemetry_stackdriver::{StackDriverExporter, YupAuthorizer};
use schema::Schema;
use sea_orm::{ConnectOptions, Database};
use std::{env, path::PathBuf, time::Duration};

#[derive(Clone)]
pub struct AppState {
    pub schema: Schema,
}

#[tokio::main]
async fn main() {
    let authorizer = YupAuthorizer::new(
        PathBuf::from("service-account.json"),
        PathBuf::from("tokens.json"),
    )
    .await
    .unwrap();

    let (exporter, driver) = StackDriverExporter::builder()
        .build(authorizer)
        .await
        .unwrap();

    tokio::spawn(driver);

    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let tracer = provider.tracer("tracing");

    let opentelemetry_extension = OpenTelemetry::new(tracer);
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

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(conn)
        .data(post_loader)
        .limit_complexity(5000)
        .extension(opentelemetry_extension)
        // .limit_depth(5)
        .finish();

    let state = AppState { schema };
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
