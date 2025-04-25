use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error::ResponseError, http::StatusCode, FromRequest};
use actix_web::web::Bytes;
use actix_web::dev::Payload;
use futures::future::{ready, Ready};
use reqwest::Client;
use std::fmt;
use reqwest::{Method, header::{HeaderMap as ReqHeaderMap, HeaderName, HeaderValue}};
use http::method::InvalidMethod;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use tracing_actix_web::TracingLogger;

#[derive(Debug)]
struct Token(String);

#[derive(Debug)]
enum AppError {
    Unauthorized,
    BadGateway(reqwest::Error),
    PayloadError(actix_web::Error),
    InvalidMethod(InvalidMethod),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::BadGateway(e) => write!(f, "Bad gateway: {}", e),
            AppError::PayloadError(e) => write!(f, "Payload error: {}", e),
            AppError::InvalidMethod(e) => write!(f, "Invalid method: {}", e),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            AppError::PayloadError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidMethod(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        AppError::PayloadError(err)
    }
}

impl FromRequest for Token {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("Authorization").and_then(|h| h.to_str().ok()) {
            Some(val) => ready(Ok(Token(val.replace("Bearer ", "")))),
            None => ready(Err(AppError::Unauthorized)),
        }
    }
}

#[instrument]
#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[instrument]
#[get("/readiness")]
async fn readiness() -> impl Responder {
    let client = Client::new();
    match client.post("https://auth-service.dndbeyond.com/v1/cobalt-token").send().await {
        Ok(res) if res.status().is_success() => HttpResponse::Ok().body("Ready"),
        _ => HttpResponse::ServiceUnavailable().body("Auth service is not ready"),
    }
}

#[instrument]
#[get("/liveliness")]
async fn liveliness() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[instrument]
#[get("/startup")]
async fn startup() -> impl Responder {
    HttpResponse::Ok().body("Started")
}

#[instrument(skip(req, body, client))]
#[get("/proxy")]
async fn proxy(req: HttpRequest, body: Bytes, client: web::Data<Client>, token: Token) -> Result<HttpResponse, AppError> {
    let method = Method::from_bytes(req.method().as_str().as_bytes()).map_err(AppError::InvalidMethod)?;
    let mut forward = client.request(method, "https://auth-service.dndbeyond.com/v1/cobalt-token")
        .header("Cookie", format!("CobaltSession={}", token.0));
    let mut headers = ReqHeaderMap::new();
    for (name, value) in req.headers().iter() {
        if let (Ok(n), Ok(val)) = (
            HeaderName::from_bytes(name.as_str().as_bytes()),
            HeaderValue::from_bytes(value.as_bytes()),
        ) {
            headers.insert(n, val);
        }
    }
    forward = forward.headers(headers);

    if !body.is_empty() {
        forward = forward.body(body.clone());
    }

    let resp = forward.send().await.map_err(AppError::BadGateway)?;
    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let bytes = resp.bytes().await.map_err(AppError::BadGateway)?;
    Ok(HttpResponse::build(status).body(bytes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Starting actix_server on 0.0.0.0:8000");
    let client = Client::new();
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(client.clone()))
            .service(healthz)
            .service(readiness)
            .service(liveliness)
            .service(startup)
            .service(proxy)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}


