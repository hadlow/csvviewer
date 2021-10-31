use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

const MAX_READ_BUFFER_SIZE: usize = 1024;

pub struct Fastq
{
    path: PathBuf,
    read_buffer: Vec<Vec<String>>,
}

impl Fastq
{
    pub fn new(path: &PathBuf) -> Self
    {
        Self
        {
            path: PathBuf::from(path),
            read_buffer: vec![vec![String::new(); 0]],
        }
    }

    pub fn tokenize(&mut self) -> io::Result<()>
    {
        let file = File::open(self.path.to_str().unwrap())?;
        let reader = BufReader::new(file);

        let mut current_read_buffer: Vec<String> = vec![String::new(); 4];
        let mut current_read_position: usize = 0;

        for wrapped_line in reader.lines()
        {
            let line = match wrapped_line
            {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            if !line.is_empty()
            {
                current_read_buffer[current_read_position] = line;
                current_read_position = current_read_position + 1;
            }

            if current_read_position == 4
            {
                self.store_read(&current_read_buffer);
                current_read_buffer = vec![String::new(); 4];
                current_read_position = 0;
            }
        }

        Ok(())
    }

    fn store_read(&mut self, read: &Vec<String>)
    {
        self.read_buffer.push(read.to_vec());

        if self.read_buffer.len() > MAX_READ_BUFFER_SIZE
        {
            self.store_read_buffer();
            self.read_buffer = vec![vec![String::new(); 0]];
        }
    }

    fn store_read_buffer(&self)
    {
        
    }
}

impl Debug for Fastq
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}
