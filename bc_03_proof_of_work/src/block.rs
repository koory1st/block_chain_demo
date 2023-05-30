use std::time::{SystemTime, UNIX_EPOCH};

use crate::proof_of_work::run;

#[derive(Debug)]
pub struct Block {
    timestamp: u64,
    data: String,
    prev_block_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let (nonce, hash) = run(prev_block_hash, data, timestamp);
        Block {
            timestamp,
            data: data.to_owned(),
            prev_block_hash: prev_block_hash.to_owned(),
            hash,
            nonce,
        }
    }
}
