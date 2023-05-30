use std::ops::ShlAssign;

use num::{BigInt, Num};
use sha256::digest;

const TARGET_BITS: u32 = 24;
pub fn run(timestamp: u64, data: &str, prev_block_hash: &str) -> (u32, String) {
    let mut target = BigInt::new(num::bigint::Sign::Plus, vec![1]);
    target.shl_assign(256 - TARGET_BITS);

    for i in 0..=u32::MAX {
        let hash = digest(format!(
            "{}{}{:x}{:x}{:x}",
            prev_block_hash, data, timestamp, TARGET_BITS, i
        ));

        let hash_int = BigInt::from_str_radix(&hash, 16).unwrap();

        if hash_int < target {
            return (i, hash);
        }
    }

    (0u32, String::from(""))
}
