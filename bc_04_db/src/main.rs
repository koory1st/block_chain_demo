use block_chain::BlockChain;

use crate::block::Block;

mod block;
mod block_chain;
mod proof_of_work;

fn main() {
    // let block = Block::new("1111", "");
    //
    // dbg!(&block);
    // println!("Hello, world!");
    //
    BlockChain::new();
}
