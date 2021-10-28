use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use crate::tokenizer;

pub struct Fastq
{
    path: String,
}

impl tokenizer::Tokenizer for Fastq
{
    fn get_file_name(&self) -> String
    {
        self.path
    }
}

impl Fastq
{
    pub fn new(path: &String) -> Self
    {
        Self
        {
            path: path,
        }
    }

    pub fn tokenize(&self) -> io::Result<()>
    {
        let file = File::open(self.filename.clone())?;
        let reader = BufReader::new(file);

        for line in reader.lines()
        {
            println!("{}", line?);
        }

        Ok(())
    }
}
