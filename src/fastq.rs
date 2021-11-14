use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

use md5;

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
        let mut current_line_buffer: Vec<u8> = vec![];
        let mut current_read_buffer: Vec<u8> = vec![];
        let mut current_read_position: u8 = 0;
        let mut last_byte: u8;

        for byte in chunk.iter()
        {
            // If reached EOL
            if *byte == 10
            {
                last_byte: u8 = match current_line_buffer.last()
                {
                    None => 0,
                    Some(b) => *b,
                };

                // If char before is EOL
                if last_byte == 10
                {

                } else {
                    current_line_buffer.push(*byte);
                    current_read_position = current_read_position + 1;
                    current_read_buffer.append(&mut current_line_buffer);
                    current_line_buffer = vec![];
                }
            } else {
                current_line_buffer.push(*byte);
            }

            if current_read_position == 4
            {
                println!("{}", String::from_utf8(current_read_buffer.clone()).unwrap());
                println!("------");
                self.store_read(&current_read_buffer);
                current_read_buffer = vec![];
                current_read_position = 0;
            }
        }

        self.store_read(&current_read_buffer);

        Ok(())
    }

    fn store_read(&mut self, read: &Vec<u8>)
    {
        self.read_buffer.push(read.clone());
        self.key_buffer.push(md5::compute(read.clone()).to_vec());
        self.shard_buffer.push(u128::from_be_bytes(md5::compute(read).0) % NUM_SHARDS);
    }
}
