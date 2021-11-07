use std::convert::Infallible;

use hyper::{Body, Response, Method, Request, StatusCode};

pub async fn routes(request: Request<Body>) -> Result<Response<Body>, Infallible>
{
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") => Ok(get()),
        (&Method::POST, "/") => Ok(post()),
        (&Method::DELETE, "/") => Ok(delete()),
        _ => Ok(error()),
    }
}

// Retrieve a file
pub fn get() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("GET");
    *response.status_mut() = StatusCode::OK;

    response
}

// Uploading a new file
pub fn post() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("POST");
    *response.status_mut() = StatusCode::CREATED;

    response
}

// Delete an existing file
pub fn delete() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("DELETE");
    *response.status_mut() = StatusCode::OK;

    response
}

pub fn error() -> Response<Body>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("ERROR");
    *response.status_mut() = StatusCode::NOT_IMPLEMENTED;

    response
}
