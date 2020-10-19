use askama::Template;
use actix_web::{Error, HttpResponse};
// Index
#[derive(Template)]
#[template(path = "index.html")]
struct Index;

pub async fn index() -> Result<HttpResponse, Error> {
    let body = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
