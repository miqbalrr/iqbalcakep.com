mod config;
mod handlers;
mod error;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
use dotenv::dotenv;
use handlers::index::index;
use handlers::grabmeta::grab_meta_handler;


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {

        App::new()
        .wrap(error::error_handlers())
        .service(Files::new("/images", "static/images/").show_files_listing())
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/grab-meta").route(web::get().to(grab_meta_handler)))
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