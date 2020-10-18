use actix_web::{error, web, Error, App, HttpResponse, HttpServer};
use listenfd::ListenFd;

use tera::Tera;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init();
    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();

        App::new()
        .data(tera)
        .service(web::resource("/").route(web::get().to(index)))
    });

    server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(val) => server.listen(val)?,
        None => server.bind("0.0.0.0:8080")?,
    };

    server.run().await
}

async fn index(
    template: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    ctx.insert("greet", &"Welcome to Iqbalcakep".to_owned());
    let body = template.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}