
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn db()
{
    let mut db = PickleDb::new("hypersquare.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
    let db2 = PickleDb::load("hypersquare.db", PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();

    if(mode == "set")
    {
        db.set(key, &100).unwrap();
        println!("Successfull set key");
    } else {
        println!("The value is: {}", db.get::<i32>(key).unwrap());
    }
}
