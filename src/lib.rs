mod config;
mod handlers;

use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
use dotenv::dotenv;
use handlers::index::*;


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {

        App::new()
        .service(web::resource("/").route(web::get().to(index)))
    });

    server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(val) => server.listen(val)?,
        None => server.bind(format!("{}:{}", config.server.host, get_server_port()))?,
    };

    server.run().await
}

fn get_server_port() -> u16 {
    std::env::var("PORT").unwrap_or("6767".to_string()).parse().unwrap()
}