use std::ops::ShlAssign;

use num::{BigInt, Num};

const TARGET_BITS: u32 = 24;
pub fn run(pre_block_hash: &str, data: &str, timestamp: u64) -> (u32, String) {
    let mut target = BigInt::new(num::bigint::Sign::Plus, vec![1]);
    target.shl_assign(256 - TARGET_BITS);

    let mut ret_nonce = 0u32;
    let mut ret_hash = String::from("");

    for i in 0..=u32::MAX {
        ret_hash = sha256::digest(format!(
            "{}{}{:x}{:x}{:x}",
            pre_block_hash, data, timestamp, TARGET_BITS, i
        ));

        let ret_hash_int = BigInt::from_str_radix(&ret_hash, 16).unwrap();

        if ret_hash_int < target {
            ret_nonce = i;
            break;
        }
    }

    (ret_nonce, ret_hash)
}
