use std::path::PathBuf;

use crate::fastq;

/*
#[derive(Debug)]
enum Tokenizer
{
    Fastq(fastq::Fastq),
    Default(bool),
}
*/

pub fn get_tokenizer(file: &str) -> fastq::Fastq
{
    let file_path = PathBuf::from(&file.to_string());
    //let extension = file_path.extension().and_then(OsStr::to_str);

    fastq::Fastq::new(&file_path)

    /*
    match extension
    {
        Some("fastq") => Tokenizer::Fastq(fastq::Fastq::new(&file_path)),
        _ => Tokenizer::Default(false),
    }
    */
}
