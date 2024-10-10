use crate::routes::{subscribe, health_check};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;



pub fn run(listener: TcpListener,
           db_pool: PgPool
          ) -> Result<Server, std::io::Error> {

    let postgrespool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
        .wrap(TracingLogger::default())
        .route("/health_check", web::get().to(health_check))
        .route("/subscribe", web::post().to(subscribe))
        .app_data(postgrespool.clone()) 
    })
    .listen(listener)?
    .run();

    Ok(server)
}