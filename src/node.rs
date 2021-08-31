extern crate dotenv;

pub struct Node
{

}

impl Node
{
    pub fn new()
    {
        
    }

    fn get_initial_nodes()
    {
        dotenv::dotenv().expect("Failed to read .env file");

        let nodes = env::var("INITIAL_NODES_INDEX").expect("INITIAL_NODES_INDEX not found");

        
    }
}
