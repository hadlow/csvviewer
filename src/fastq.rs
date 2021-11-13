use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

use hyper::{Body};

use md5;

const MAX_READ_BUFFER_SIZE: usize = 2;
const NUM_SHARDS: u128 = 4;

pub struct Fastq
{
    read_buffer: Vec<Vec<u8>>,
    key_buffer: Vec<Vec<u8>>,
    shard_buffer: Vec<u128>,
}

impl Fastq
{
    pub fn new() -> Self
    {
        Self
        {
            read_buffer: vec![vec![0; 0]; 0],
            key_buffer: vec![vec![0; 0]; 0],
            shard_buffer: vec![0; 0],
        }
    }

    pub fn tokenize(&mut self, chunk: &hyper::body::Bytes) -> io::Result<()>
    {
        let mut current_read_buffer: Vec<u8> = vec![0; 4];
        let mut current_read_position: usize = 0;

        for byte in chunk.iter()
        {
            current_read_buffer.push(*byte);

            println!("{}", byte);
            current_read_position = current_read_position + 1;

            /*
            if current_read_position == 4
            {
                self.store_read(&current_read_buffer);
                current_read_buffer = vec![String::new(); 4];
                current_read_position = 0;
            }
            */
        }

        self.store_read(&current_read_buffer);

        Ok(())
    }

    fn store_read(&mut self, read: &Vec<u8>)
    {
        self.read_buffer.push(read.clone());
        self.key_buffer.push(md5::compute(read.clone()).to_vec());
        self.shard_buffer.push(u128::from_be_bytes(md5::compute(read).0) % NUM_SHARDS);

        // Once we've hit the buffer limit, broadcast reads to their nodes
        if self.read_buffer.len() >= MAX_READ_BUFFER_SIZE
        {
            // Reset all buffers
            self.read_buffer = vec![vec![0; 0]; 0];
            self.key_buffer = vec![vec![0; 0]; 0];
            self.shard_buffer = vec![0; 0];
        }
    }
}
