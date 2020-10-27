use actix_http::{body::Body, Response};
use actix_web::dev::{ServiceResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use askama::Template;
use actix_web::{Result, error, http::header, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum IqbalCakepError {
    InternalError,
    BadClientData,
    Timeout
}

impl error::ResponseError for IqbalCakepError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            IqbalCakepError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            IqbalCakepError::BadClientData => StatusCode::BAD_REQUEST,
            IqbalCakepError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

// Index
#[derive(Template)]
#[template(path = "404.html")]
struct NotFound;

// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body>{ 

        let fallback = |e: &str| {
            Response::build(res.status())
                .content_type("text/plain")
                .body(e.to_string())
        };
       let body = NotFound.render();
       match body {
        Ok(body) => Response::build(res.status())
            .content_type("text/html")
            .body(body),
        Err(_) => fallback(error),
      }
}