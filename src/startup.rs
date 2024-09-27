use crate::routes::{subscribe, health_check};
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web::middleware::Logger;


pub fn run(listener: TcpListener,
           db_pool: PgPool
          ) -> Result<Server, std::io::Error> {

    let postgrespool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
        .wrap(Logger::new("%a %{User-Agent}i"))
        .route("/health_check", web::get().to(health_check))
        .route("/subscribe", web::post().to(subscribe))
        .app_data(postgrespool.clone()) 
    })
    .listen(listener)?
    .run();

    Ok(server)
}