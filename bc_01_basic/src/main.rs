use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use sha256::digest;
fn main() {
    let mut block_chain = BlockChain::new();

    block_chain.add_block("1st");
    block_chain.add_block("2nd");

    dbg!(block_chain);

    println!("Hello, world!");
}

#[derive(Debug)]
struct Block {
    timestamp: u64,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {
    fn new(data: &str, prev_block_hash: &str) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = format!(
            "{}{}{}",
            prev_block_hash,
            data,
            format!("{:010}", timestamp)
        );

        let hash = digest(hash);
        Block {
            timestamp,
            data: data.to_owned(),
            prev_block_hash: prev_block_hash.to_owned(),
            hash,
        }
    }
}

#[derive(Debug)]
struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        BlockChain {
            blocks: vec![Block::new("Genesis Block", "")],
        }
    }
    fn add_block(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        let prev_block: &Block = self.blocks.last().expect("No Prev Block!");
        let new_block = Block::new(data, prev_block.hash.as_ref());

        self.blocks.push(new_block);
        Ok(())
    }
}
