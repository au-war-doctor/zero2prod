use zero2prod::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read from config file");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    // let listener = TcpListener::bind("127.0.0.1:0")
    //     .expect("Failed to bind to an available port");

    let listener = TcpListener::bind(address)?;
    run(listener)?.await    
}

