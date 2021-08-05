extern crate clap;

use clap::{Arg, App};
use tokio::{io::AsyncReadExt, net::TcpListener};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

#[tokio::main]
async fn main()
{
    let matches = App::new("Hypersquare")
        .version("0.1.0")
        .author("Billy Hadlow")
        .about("")
        .arg(Arg::with_name("test")
             .short("t")
             .long("test")
             .value_name("T")
             .help("This is a test command")
             .takes_value(true))
        .get_matches();

    let test = matches.value_of("test").unwrap_or("no test entered");

    println!("Test is: {}", test);
}
