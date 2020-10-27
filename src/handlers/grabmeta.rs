use serde_derive::Deserialize;
use actix_web::{Error, web, HttpResponse};
use grab_meta::*;
use actix_web::http::StatusCode;
use tokio::runtime::Builder;


#[derive(Deserialize)]
pub struct Params {
    url: String
}

pub async fn grab_meta_handler(
    web::Query(info) : web::Query<Params>
) -> Result<HttpResponse, Error> {
    let rt  = Builder::new_multi_thread().build().unwrap();
    rt.block_on(async {
        let meta_data : tokio::task::JoinHandle<Result<meta::Meta, meta_error::MetaError>> = tokio::task::spawn(async move {
            get_meta(info.url.as_str())
        });
        let result = meta_data.await.unwrap();
        match result {
            Ok(data) => {
                return Ok(HttpResponse::Ok().json(data)); },
            Err(_) => { 
                return  Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).finish());
            }
        }
    })
}