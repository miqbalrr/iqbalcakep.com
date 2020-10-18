use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(val) => server.listen(val)?,
        None => server.bind("127.0.0.1:2828")?,
    };

    server.run().await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Iqbalcakep.com is running :) ")
}