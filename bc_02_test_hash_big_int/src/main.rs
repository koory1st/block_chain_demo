use num::{BigInt, Num};
use sha256::digest;
use std::ops::ShlAssign;

fn main() {
    println!("Hello, world!");

    let target_bits = 24;

    let mut target = BigInt::from(1);

    target.shl_assign(256 - target_bits);

    let data = "I like donuts";

    let hash = digest(data);

    dbg!(hash);

    let hash_2 = digest(format!("{}{}", data, "sha256"));
    dbg!(BigInt::from_str_radix(&hash_2, 16));

    for nonce in 0..=i32::MAX {
        let hash_new = digest(format!("{}{:x}", data, nonce));
        let hash_new = BigInt::from_str_radix(&hash_new, 16).unwrap();

        if hash_new < target {
            dbg!(nonce);
            break;
        }
    }
}
