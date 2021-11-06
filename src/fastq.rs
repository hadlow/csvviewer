use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

use md5;

const MAX_READ_BUFFER_SIZE: usize = 2;
const NUM_SHARDS: u128 = 4;

struct Read
{
    key: Vec<u8>,
    data: Vec<u8>,
    shard: u128,
}

pub struct Fastq
{
    path: PathBuf,
    read_buffer: Vec<Vec<u8>>,
    key_buffer: Vec<Vec<u8>>,
    shard_buffer: Vec<u128>,
}

impl Fastq
{
    pub fn new(path: &PathBuf) -> Self
    {
        Self
        {
            path: PathBuf::from(path),
            read_buffer: vec![vec![0; 0]; 0],
            key_buffer: vec![vec![0; 0]; 0],
            shard_buffer: vec![0; 0],
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
        let read_as_bytes: Vec<u8> = read.join("\n").as_bytes().to_vec();

        self.read_buffer.push(read_as_bytes.clone());
        self.key_buffer.push(md5::compute(read_as_bytes.clone()).to_vec());
        self.shard_buffer.push(u128::from_be_bytes(md5::compute(read_as_bytes).0) % NUM_SHARDS);

        // Once we've hit the buffer limit, broadcast reads to their nodes
        if self.read_buffer.len() >= MAX_READ_BUFFER_SIZE
        {
            self.broadcast_read_buffer();

            // Reset all buffers
            self.read_buffer = vec![vec![0; 0]; 0];
            self.key_buffer = vec![vec![0; 0]; 0];
            self.shard_buffer = vec![0; 0];
        }
    }

    fn broadcast_read_buffer(&self)
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
