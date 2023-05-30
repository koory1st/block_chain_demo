mod block;
mod proof_of_work;
use std::ops::{Shl, ShlAssign};

use block::Block;
use num::BigInt;

fn main() {
    println!("Hello, world!");

    let block: Block = Block::new("111", "");

    dbg!(&block);
}
