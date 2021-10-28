use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub struct Tokenizer
{
    filename: String,
}

impl Tokenizer
{
    pub fn new(filename: &str) -> Self
    {
        Self
        {
            filename: filename.to_string(),
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
