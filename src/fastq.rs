use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

const MAX_READ_BUFFER_SIZE: usize = 2;
const NUM_SHARDS: usize = 4;

pub struct Fastq
{
    path: PathBuf,
    read_buffer: Vec<Vec<u8>>,
}

impl Fastq
{
    pub fn new(path: &PathBuf) -> Self
    {
        Self
        {
            path: PathBuf::from(path),
            read_buffer: vec![vec![0; 0]; 0],
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

        self.store_read(&current_read_buffer);

        Ok(())
    }

    fn store_read(&mut self, read: &Vec<String>)
    {
        self.read_buffer.push(read.join("\n").as_bytes().to_vec());

        if self.read_buffer.len() >= MAX_READ_BUFFER_SIZE
        {
            for line in self.read_buffer.iter()
            {
                println!("{:?}", line);
            }
            println!("---------------------------------------------------------------------");
            self.write_read_buffer(&self.shard_read_buffer());
            self.read_buffer = vec![vec![0; 0]; 0];
        }
    }

    fn shard_read_buffer(&self) -> Vec<Vec<u8>>
    {
        let mut shards: Vec<Vec<u8>> = vec![vec![0; 0]; NUM_SHARDS];

        vec![vec![0]]
    }

    fn write_read_buffer(&self, shards: &Vec<Vec<u8>>)
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
