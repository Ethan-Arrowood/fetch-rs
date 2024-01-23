#![deny(clippy::all)]

use ada_url::Url as AdaUrl;
use napi::bindgen_prelude::Null;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
#[derive(Clone)]
pub struct Url {
  // pub href: String,
  // pub origin: String,
  // pub protocol: String,
  // pub username: String,
  // pub password: String,
  pub host: String,
  // pub hostname: String,
  // pub port: String,
  // pub pathname: String,
  // pub search: String,
  // pub SearchParams: Null,
  // pub hash: String
}

#[napi]
pub struct Request {
  pub url: Url,
}

#[napi]
impl Request {
  #[napi(constructor)]
  pub fn new(init: String) -> Self {
    let parsedUrl = AdaUrl::parse(&init, None).expect("bad url");
    Request { url: Url { host: parsedUrl.host().to_string() } }
  }
}

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
pub async fn fetch_with_request(resource: &Request) -> napi::Result<Response> {
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
  resource: &Request,
  init: RequestInit,
) -> napi::Result<Response> {
  println!("fetching with request and init");
  Ok(Response {})
}
