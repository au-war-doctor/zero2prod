use crate::routes::{subscribe, health_check};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;


pub fn run(listener: TcpListener,
           db_pool: PgPool
          ) -> Result<Server, std::io::Error> {

    let postgrespool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscribe", web::post().to(subscribe))
        .app_data(postgrespool.clone()) // TODO! check if this should be db_pool? No, right?
    })
    .listen(listener)?
    .run();

    Ok(server)
}