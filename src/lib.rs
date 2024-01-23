#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Request {}

#[napi(object)]
pub struct RequestInit {}

#[napi(object)]
pub struct Response {}

#[napi]
pub async fn fetch_with_str(resource: String) -> napi::Result<Response> {
  println!("fetching with str {}", resource);
  Ok(Response {})
}

#[napi]
pub async fn fetch_with_request(resource: Request) -> napi::Result<Response> {
  println!("fetching with request");
  Ok(Response {})
}

#[napi]
pub async fn fetch_with_str_and_init(
  resource: String,
  init: RequestInit,
) -> napi::Result<Response> {
  println!("fetching with str and init");
  Ok(Response {})
}

#[napi]
pub async fn fetch_with_request_and_init(
  resource: Request,
  init: RequestInit,
) -> napi::Result<Response> {
  println!("fetching with request and init");
  Ok(Response {})
}
