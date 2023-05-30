use std::time::{SystemTime, UNIX_EPOCH};

use crate::proof_of_work;

#[derive(Debug)]
pub struct Block {
    timestamp: u64,
    prev_block_hash: String,
    data: String,
    hash: String,
    nonce: u32,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Block {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let (nonce, hash) = proof_of_work::run(timestamp, data, prev_block_hash);

        Block {
            timestamp,
            prev_block_hash: prev_block_hash.to_owned(),
            data: data.to_owned(),
            hash,
            nonce,
        }
    }
}
