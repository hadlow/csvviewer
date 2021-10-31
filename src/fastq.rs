use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

pub struct Fastq
{
    path: PathBuf,
}

impl Fastq
{
    pub fn new(path: &PathBuf) -> Self
    {
        Self
        {
            path: PathBuf::from(path),
        }
    }

    pub fn tokenize(&self) -> io::Result<()>
    {
        let file = File::open(self.path.to_str().unwrap())?;
        let reader = BufReader::new(file);

        let mut read_buffer: Vec<String> = vec![String::new(); 4];
        let mut current_read: usize = 0;

        for wrapped_line in reader.lines()
        {
            let line = match wrapped_line
            {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            if !line.is_empty()
            {
                read_buffer[current_read] = line;
                current_read = current_read + 1;
            }

            if current_read == 4
            {
                println!("{}", read_buffer[0]);

                read_buffer = vec![String::new(); 4];
                current_read = 0;
            }
        }

        Ok(())
    }
}

impl Debug for Fastq
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}
