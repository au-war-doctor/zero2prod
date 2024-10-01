use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //redirect all log events to subscriber
    LogTracer::init().expect("Failed to set logger");

    //set default logging to info
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // set default to use this subscriber
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
   

    let configuration = get_configuration().expect("Failed to read from config file");    
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await    
}

