#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Request {

}

#[napi(object)]
pub struct RequestInit {

}

#[napi]
pub fn fetch_with_str(resource: String) -> () {
  println!("fetching with str {}", resource)
}

#[napi]
pub fn fetch_with_request(resource: Request) -> () {
  println!("fetching with request")
}

#[napi]
pub fn fetch_with_str_and_init(resource: String, init: RequestInit) -> () {
  println!("fetching with str and init")
}

#[napi]
pub fn fetch_with_request_and_init(resource: Request, init: RequestInit) -> () {
  println!("fetching with request and init")
}