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

    let mut db = PickleDb::new("hypersquare.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
    let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();

    if(mode == "set")
    {
        db.set(key, &100).unwrap();
        println!("Successfull set key");
    } else {
        println!("The value is: {}", db.get::<i32>(key).unwrap());
    }
}
