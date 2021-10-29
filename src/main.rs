extern crate clap;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::ffi::OsStr;
use std::path::PathBuf;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use clap::{Arg, App};
/*
use tokio::{
    io::AsyncBufReadExt,
    io::AsyncReadExt,
    io::AsyncWriteExt,
    io::BufReader,
    net::TcpListener,
    net::TcpStream
};
*/

mod fastq;

async fn connect(req: Request<Body>) -> Result<Response<Body>, Infallible>
{
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

#[derive(Debug)]
enum Tokenizer
{
    Fastq(fastq::Fastq),
    Default(bool),
}

fn get_tokenizer(file: &str) -> Tokenizer
{
    let file_path = PathBuf::from(&file.to_string());
    let extension = file_path.extension().and_then(OsStr::to_str);

    match extension
    {
        Some("fastq") => Tokenizer::Fastq(fastq::Fastq::new(&file_path)),
        _ => Tokenizer::Default(false),
    }
}

#[tokio::main]
async fn main()
{
    let matches = App::new("GenomDB")
        .version("0.1.0")
        .author("Billy Hadlow")
        .about("")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .help("FASTQ file")
             .required(false)
             .takes_value(true))
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

    let _shard = matches.value_of("shard").unwrap_or("No shard ID has been provided");
    let file = matches.value_of("file").unwrap_or("No file name has been provided");
    let port = matches.value_of("port").unwrap_or("No port has been provided");
    let _config = matches.value_of("config").unwrap_or("No config file has been provided");

    let tokenizer = get_tokenizer(file);

    println!("{:?}", get_tokenizer(file));

    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap()));

    let make_svc = make_service_fn(|_conn| async
    {
        Ok::<_, Infallible>(service_fn(connect))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await
    {
        eprintln!("server error: {}", e);
    }
}

/*
let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

loop
{
    let (mut socket, mut address) = listener.accept().await.unwrap();

    tokio::spawn(async move
    {
        let (read, mut write) = socket.split();

        let mut buffer = BufReader::new(read);
        let mut line = String::new();

        loop
        {
            let bytes_read = buffer.read_line(&mut line).await.unwrap();

            if bytes_read == 0
            {
                break;
            }

            write.write_all(line.as_bytes()).await.unwrap();
            line.clear();
        }
    });
}
*/
