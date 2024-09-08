use std::sync::Arc;
use dotenvy;
use envy;
use serde::{Deserialize, Serialize};
use axum::{
    response::{IntoResponse, Html},
    routing::{get},
    Router,
    extract::{Query, State},
};
use tokio_postgres::{self, NoTls};
use log::{log, Level};

#[derive(Deserialize)]
struct Config {
    #[serde(default = "db_default_user")]
    postgres_user: String,
    #[serde(default = "db_default_password")]
    postgres_password: String,
    #[serde(default = "db_default_name")]
    postgres_db: String,
    #[serde(default = "db_default_host")]
    postgres_host: String,
    #[serde(default = "db_default_port")]
    postgres_port: String,
    #[serde(default = "server_default_address")]
    address: String,
}

impl Config {
    fn get_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.postgres_user, self.postgres_password, self.postgres_host, self.postgres_port, self.postgres_db)
    }
}

struct DataBasePostgres {
    db: tokio_postgres::Client,
}

fn server_default_address() -> String {
    "localhost:8000".to_string()
}

fn db_default_user() -> String {
    "postgres".to_string()
}

fn db_default_password() -> String {
    "postgres".to_string()
}

fn db_default_name() -> String {
    "postgres".to_string()
}

fn db_default_host() -> String {
    "localhost".to_string()
}

fn db_default_port() -> String {
    "5432:5432".to_string()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Can't read .env");

    let config = envy::from_env::<Config>().unwrap_or_else(|err| {
        eprintln!("Can't parse .env: {}", err);
        std::process::exit(1);
    });

    run(config).await;
}

async fn run(config: Config) {
    let (client, connection) = tokio_postgres::connect(&config.get_url(), NoTls)
        .await.unwrap_or_else(|err| {
        eprintln!("Can't connect to postgres: {}", err);
        std::process::exit(1);
    });

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });
    log!(Level::Info, "Connected to data base {}", config.get_url());

    let app = routes(client);
    let listener = tokio::net::TcpListener::bind(&config.address).await.unwrap_or_else(|err| {
        eprintln!("Can't bind to localhost:8000: {}", err);
        std::process::exit(1);
    });

    axum::serve(listener, app).await.expect("Can't start server");
    log!(Level::Info, "Server starts at {}", config.address);
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiParams {
    id: Option<i32>,
    name: Option<String>,
}

fn routes(client: tokio_postgres::Client) -> Router {
    let app_state = Arc::new(DataBasePostgres { db: client });

    Router::new()
        .route("/api", get(handler))
        .with_state(app_state)
}

async fn handler(
    State(data): State<Arc<DataBasePostgres>>,
    Query(params): Query<ApiParams>,
) -> impl IntoResponse {
    let rows = data.db.query("SELECT name FROM Users WHERE id=$1", &[&params.id.unwrap_or(-1)])
        .await.expect("Can't select main");
    println!("{:?}", params);
    let name: String = rows[0].get(0);
    Html(format!("You search: {}\nName is: {}", params.id.unwrap_or(-1), name))
}

