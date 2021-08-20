extern crate clap;

use clap::{Arg, App};
use tokio::{
    io::AsyncBufReadExt,
    io::AsyncReadExt,
    io::AsyncWriteExt,
    io::BufReader,
    net::TcpListener,
    net::TcpStream
};

#[tokio::main]
async fn main()
{
    let matches = App::new("Hypersquare")
        .version("0.1.0")
        .author("Billy Hadlow")
        .about("")
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
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .help("Key value")
             .required(true)
             .takes_value(true))
        .get_matches();

    let _shard = matches.value_of("shard").unwrap_or("No shard ID has been provided");
    let _port = matches.value_of("port").unwrap_or("No port has been provided");
    let _config = matches.value_of("config").unwrap_or("No config file has been provided");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let (mut socket, mut address) = listener.accept().await.unwrap();

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
}
