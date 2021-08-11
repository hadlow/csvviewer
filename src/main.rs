extern crate clap;

use clap::{Arg, App};
use tokio::{io::AsyncReadExt, net::TcpListener};

mod db;

#[tokio::main]
async fn main()
{
    let matches = App::new("Hypersquare")
        .version("0.1.0")
        .author("Billy Hadlow")
        .about("")
        .arg(Arg::with_name("mode")
             .help("Get/set")
             .required(true)
             .index(1))
        .arg(Arg::with_name("key")
             .help("Key name")
             .required(true)
             .index(2))
        .arg(Arg::with_name("value")
             .help("Key value")
             .required(false)
             .index(3))
        .get_matches();

    let mode = matches.value_of("mode").unwrap_or("No mode has been provided");
    let key = matches.value_of("key").unwrap_or("No key has been provided");
    let value = matches.value_of("value").unwrap_or("No value has been provided");

}
