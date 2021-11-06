use hyper::{Body, Response};

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
