use std::sync::Arc;
use dotenvy;
use envy;
use serde::{Deserialize, Serialize};
use axum::{response::{IntoResponse, Json}, routing::{get}, Router, extract::{Query, State}};
use axum::http::StatusCode;
use tokio_postgres::{self, NoTls};
use log::{log, Level};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    uid: Option<String>,
}

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

impl DataBasePostgres {
    async fn get_order_by_uid(&self, uid: &String) -> Result<Order, String> {
        let mut order_builder = Order::builder();
        order_builder.with_order_uid(uid.clone());

        let delivery_result_statement = self.db.query(
            "SELECT * FROM delivery
            WHERE order_uid=$1", &[&uid]
        ).await;

        let delivers = match delivery_result_statement {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string()),
        };

        if delivers.is_empty() {
            return Err("No order found".to_string());
        }

        if delivers.len() > 1 {
            return Err(format!("Order {} has more than one delivery", uid));
        }

        order_builder.with_delivery(Delivery {
            name: delivers[0].get("name"),
            phone: delivers[0].get("phone"),
            zip: delivers[0].get("zip"),
            city: delivers[0].get("city"),
            address: delivers[0].get("address"),
            region: delivers[0].get("region"),
            email: delivers[0].get("email"),
        });

        let payment_result_statement = self.db.query(
            "SELECT * FROM payments
            WHERE transaction=$1", &[&uid]
        ).await;

        let payments = match payment_result_statement {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string()),
        };

        if payments.is_empty() {
            return Err("No order found".to_string());
        }

        if payments.len() > 1 {
            return Err(format!("Order {} has more than one delivery", uid));
        }

        order_builder.with_payment(Payment {
            transaction: payments[0].get("transaction"),
            request_id: payments[0].get("request_id"),
            currency: payments[0].get("currency"),
            provider: payments[0].get("provider"),
            amount: payments[0].get("amount"),
            payment_dt: payments[0].get("payment_dt"),
            bank: payments[0].get("bank"),
            delivery_cost: payments[0].get("delivery_cost"),
            goods_total: payments[0].get("goods_total"),
            custom_fee: payments[0].get("custom_fee"),
        });

        let items_result_statement = self.db.query(
            "SELECT * FROM products
            WHERE order_uid=$1", &[&uid]
        ).await;

        let items = match items_result_statement {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string()),
        };

        if items.is_empty() {
            return Err("No order found".to_string());
        }

        let mut items_vec = Vec::new();
        for row in items {
            items_vec.push(Item {
                chrt_id: row.get("chrt_id"),
                track_number: row.get("track_number"),
                price: row.get("price"),
                rid: row.get("rid"),
                name: row.get("name"),
                sale: row.get("sale"),
                size: row.get("size"),
                total_price: row.get("total_price"),
                nm_id: row.get("nm_id"),
                brand: row.get("brand"),
                status: row.get("status"),
            });
        }

        order_builder.with_item(items_vec);

        let order_result_statement = self.db.query(
            "SELECT * FROM orders
            WHERE order_uid=$1", &[&uid]
        ).await;

        let orders = match order_result_statement {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string()),
        };

        if orders.is_empty() {
            return Err("No order found".to_string());
        }

        if orders.len() > 1 {
            return Err(format!("Order {} has more than one delivery", uid));
        }

        order_builder.with_track_number(orders[0].get("track_number"));
        order_builder.with_entry(orders[0].get("entry"));
        order_builder.with_locale(orders[0].get("locale"));
        order_builder.with_internal_signature(orders[0].get("internal_signature"));
        order_builder.with_customer_id(orders[0].get("customer_id"));
        order_builder.with_delivery_service(orders[0].get("delivery_service"));
        order_builder.with_shardkey(orders[0].get("shardkey"));
        order_builder.with_sm_id(orders[0].get("sm_id"));
        order_builder.with_date_created(orders[0].get("date_created"));
        order_builder.with_oof_shard(orders[0].get("oof_shard"));

        log!(Level::Info, "Success get request to db");

        Ok(order_builder.build())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Payment {
    transaction: String,
    request_id: Option<String>,
    currency: String,
    provider: String,
    amount: i64,
    payment_dt: i64,
    bank: String,
    delivery_cost: i64,
    goods_total: i64,
    custom_fee: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    chrt_id: i64,
    track_number: String,
    price: i64,
    rid: String,
    name: String,
    sale: i64,
    size: String,
    total_price: i64,
    nm_id: i64,
    brand: String,
    status: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: Option<String>,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i64,
    date_created: std::time::SystemTime,
    oof_shard: String,
}

impl Order {
    fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }
}

struct OrderBuilder {
    order_uid: Option<String>,
    track_number: Option<String>,
    entry: Option<String>,
    delivery: Option<Delivery>,
    payment: Option<Payment>,
    items: Option<Vec<Item>>,
    locale: Option<String>,
    internal_signature: Option<String>,
    customer_id: Option<String>,
    delivery_service: Option<String>,
    shardkey: Option<String>,
    sm_id: Option<i64>,
    date_created: Option<std::time::SystemTime>,
    oof_shard: Option<String>,
}

impl OrderBuilder {
    fn default() -> Self {
        OrderBuilder {
            order_uid: None,
            track_number: None,
            entry: None,
            delivery: None,
            payment: None,
            items: None,
            locale: None,
            internal_signature: None,
            customer_id: None,
            delivery_service: None,
            shardkey: None,
            sm_id: None,
            date_created: None,
            oof_shard: None,
        }
    }

    fn with_order_uid(&mut self, order_uid: String) -> &Self {
        self.order_uid = Some(order_uid);
        self
    }

    fn with_payment(&mut self, payment: Payment) -> &Self {
        self.payment = Some(payment);
        self
    }

    fn with_item(&mut self, items: Vec<Item>) -> &Self {
        self.items = Some(items);
        self
    }

    fn with_delivery(&mut self, delivery: Delivery) -> &Self {
        self.delivery = Some(delivery);
        self
    }

    fn with_track_number (&mut self, track_number: String) -> &Self {
        self.track_number = Some(track_number);
        self
    }

    fn with_entry (&mut self, entry: String) -> &Self {
        self.entry = Some(entry);
        self
    }

    fn with_locale (&mut self, locale: String) -> &Self {
        self.locale = Some(locale);
        self
    }

    fn with_internal_signature (&mut self, internal_signature: String) -> &Self {
        self.internal_signature = Some(internal_signature);
        self
    }

    fn with_customer_id (&mut self, customer_id: String) -> &Self {
        self.customer_id = Some(customer_id);
        self
    }

    fn with_delivery_service (&mut self, delivery_service: String) -> &Self {
        self.delivery_service = Some(delivery_service);
        self
    }

    fn with_shardkey (&mut self, shardkey: String) -> &Self {
        self.shardkey = Some(shardkey);
        self
    }

    fn with_sm_id (&mut self, sm_id: i64) -> &Self {
        self.sm_id = Some(sm_id);
        self
    }

    fn with_date_created (&mut self, date_created: std::time::SystemTime) -> &Self {
        self.date_created = Some(date_created);
        self
    }

    fn with_oof_shard (&mut self, oof_shard: String) -> &Self {
        self.oof_shard = Some(oof_shard);
        self
    }

    fn build(self) -> Order {
        Order {
            order_uid: self.order_uid.unwrap(),
            track_number: self.track_number.unwrap(),
            entry: self.entry.unwrap(),
            delivery: self.delivery.unwrap(),
            payment: self.payment.unwrap(),
            items: self.items.unwrap(),
            locale: self.locale.unwrap(),
            internal_signature: self.internal_signature,
            customer_id: self.customer_id.unwrap(),
            delivery_service: self.delivery_service.unwrap(),
            shardkey: self.shardkey.unwrap(),
            sm_id: self.sm_id.unwrap(),
            date_created: self.date_created.unwrap(),
            oof_shard: self.oof_shard.unwrap()
        }
    }
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

fn routes(client: tokio_postgres::Client) -> Router {
    let app_state = Arc::new(DataBasePostgres { db: client });

    Router::new()
        .route("/api/get-order", get(get_order_by_uid))
        .with_state(app_state)
}

async fn get_order_by_uid(
    State(data): State<Arc<DataBasePostgres>>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let uid = params.uid.unwrap_or("-1".to_string());

    let order = match data.get_order_by_uid(&uid).await {
        Ok(order) => order,
        Err(err) => {
            eprintln!("Can't get order by uid: {}", &uid);
            return Err((StatusCode::NOT_FOUND, Json(json!(err))));
        }
    };

    Ok(Json(order))
}