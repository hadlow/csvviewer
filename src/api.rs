use std::net::SocketAddr;
use std::convert::Infallible;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

pub fn get() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("GET");

    response
}


pub fn post() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("POST");

    response
}

pub fn delete() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("DELETE");

    response
}

pub fn error() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("ERROR");

    response
}
