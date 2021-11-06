extern crate clap;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::ffi::OsStr;
use std::path::PathBuf;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use clap::{Arg, App};

mod fastq;
mod api;

#[tokio::main]
async fn main()
{
    let matches = App::new("GenomDB")
        .version("0.1.0")
        .author("Billy Hadlow")
        .about("GenomDB")
        .arg(Arg::with_name("shard")
             .short("s")
             .long("shard")
             .help("The shard ID")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .help("Port to host the shard on")
             .required(true)
             .takes_value(true))
        .get_matches();

    let shard = matches.value_of("shard").unwrap_or("No shard ID has been provided");
    let port = matches.value_of("port").unwrap_or("No port has been provided");
    let _config = matches.value_of("config").unwrap_or("No config file has been provided");
    let file = "test.fastq";

    let mut tokenizer = get_tokenizer(file);
    tokenizer.tokenize();

    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap()));

    let make_svc = make_service_fn(|_conn| async
    {
        Ok::<_, Infallible>(service_fn(serve))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await
    {
        eprintln!("server error: {}", e);
    }
}

async fn serve(req: Request<Body>) -> Result<Response<Body>, Infallible>
{
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(api::get()),
        (&Method::POST, "/") => Ok(api::post()),
        (&Method::DELETE, "/") => Ok(api::delete()),
        _ => Ok(api::error()),
    }
}

#[derive(Debug)]
enum Tokenizer
{
    Fastq(fastq::Fastq),
    Default(bool),
}

fn get_tokenizer(file: &str) -> fastq::Fastq
{
    let file_path = PathBuf::from(&file.to_string());
    let extension = file_path.extension().and_then(OsStr::to_str);

    fastq::Fastq::new(&file_path)

    /*
    match extension
    {
        Some("fastq") => Tokenizer::Fastq(fastq::Fastq::new(&file_path)),
        _ => Tokenizer::Default(false),
    }
    */
}
