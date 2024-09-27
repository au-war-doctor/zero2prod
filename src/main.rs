use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};
use std::net::TcpListener;
use env_logger::Env;
use log::{debug, error, log_enabled, info, Level};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // as usual his impl is night/day from the docs
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
   

    let configuration = get_configuration().expect("Failed to read from config file");

    
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await    
}

