#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::{get, routes};
use reqwest::Client;

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => Outcome::Success(Token(token.replace("Bearer ", "").to_string())),
            None => Outcome::Error((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}

#[get("/healthz")]
fn healthz() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "OK")
}

#[get("/readiness")]
async fn readiness() -> Result<status::Custom<&'static str>, status::Custom<&'static str>> {
    let client = Client::new();
    let response = client.post("https://auth-service.dndbeyond.com/v1/cobalt-token")
        .send()
        .await
        .map_err(|_| status::Custom(Status::ServiceUnavailable, "Failed to reach auth service"))?;

    if response.status().is_success() {
        Ok(status::Custom(Status::Ok, "Ready"))
    } else {
        Err(status::Custom(Status::ServiceUnavailable, "Auth service is not ready"))
    }
}

#[get("/liveliness")]
fn liveliness() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "Alive")
}

#[get("/startup")]
fn startup() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "Started")
}

#[get("/proxy")]
async fn proxy(token: Token) -> Result<status::Custom<String>, status::Custom<&'static str>> {
    let client = Client::new();
    let response = client.post("https://auth-service.dndbeyond.com/v1/cobalt-token")
        .header("Cookie", format!("CobaltSession={}", token.0))
        .send()
        .await
        .map_err(|_| status::Custom(Status::InternalServerError, "Failed to request token"))?;

    let status = Status::from_code(response.status().as_u16()).unwrap_or(Status::InternalServerError);
    let body = response.text().await.map_err(|_| status::Custom(Status::InternalServerError, "Failed to read response body"))?;

    Ok(status::Custom(status, body))
}

#[launch]
fn rocket() -> _ {
    let mount = std::env::var("ROCKET_MOUNT").unwrap_or("/".to_string());
    rocket::build()
        .mount(&mount, routes![healthz, readiness, liveliness, startup, proxy])
}
