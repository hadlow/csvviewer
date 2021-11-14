use hyper::{Body, Response, Method, Request, StatusCode};
use futures::stream::{self, StreamExt, TryStreamExt};

use crate::tokenizer;

pub async fn routes(request: Request<Body>) -> Result<Response<Body>, hyper::Error>
{
    match (request.method(), request.uri().path())
    {
        (&Method::GET, "/") => get(request),
        (&Method::POST, "/") => post(request),
        (&Method::DELETE, "/") => delete(request),
        _ => error(request),
    }
}

// Retrieve a file
pub fn get(request: Request<Body>) -> Result<Response<Body>, hyper::Error>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("GET");
    *response.status_mut() = StatusCode::OK;

    Ok(response)
}

// Uploading a new file
pub fn post(request: Request<Body>) -> Result<Response<Body>, hyper::Error>
{
    let mut tokenizer = tokenizer::get_tokenizer();

    let mapping = request
        .into_body()
        .map_ok(move |chunk|
        {
            // Get vector of transactions to send to each node
            let transactions = tokenizer.tokenize(&chunk);

            for transaction in transactions
            {
                
            }
            
            vec![49]
        }
    );

    let mut response = Response::new(Body::empty());
    //*response.body_mut() = Body::from("POST");
    *response.body_mut() = Body::wrap_stream(mapping);
    *response.status_mut() = StatusCode::CREATED;

    Ok(response)
}

// Delete an existing file
pub fn delete(request: Request<Body>) -> Result<Response<Body>, hyper::Error>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("DELETE");
    *response.status_mut() = StatusCode::OK;

    Ok(response)
}

pub fn error(request: Request<Body>) -> Result<Response<Body>, hyper::Error>
{
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("ERROR");
    *response.status_mut() = StatusCode::NOT_IMPLEMENTED;

    Ok(response)
}
