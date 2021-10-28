use std::fs;

pub struct File
{
    contents: str[]
}

impl File
{
    pub fn new() -> Self
    {
        let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

        Self
        {
            contents: contents,
        }
    }
}
