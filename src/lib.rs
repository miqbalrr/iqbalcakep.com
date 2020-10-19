mod config;
use actix_web::{error, web, Error, App, HttpResponse, HttpServer};
use listenfd::ListenFd;
use dotenv::dotenv;
use tera::Tera;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        let tera = match Tera::new("views/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
        .data(tera)
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

async fn index(
    template: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    ctx.insert("greet", &"Welcome to Iqbalcakep".to_owned());
    let body = template.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}