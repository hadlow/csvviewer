extern crate clap;

use clap::{Arg, App};
use tokio::{io::AsyncReadExt, net::TcpListener};

// A distributed key-value DB is made up of the following compontents:
// - Networking
// - Sharding
// - Resharding
// - Data storage

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

    let shard = matches.value_of("shard").unwrap_or("No shard ID has been provided");
    let port = matches.value_of("port").unwrap_or("No port has been provided");
    let config = matches.value_of("config").unwrap_or("No config file has been provided");
}
