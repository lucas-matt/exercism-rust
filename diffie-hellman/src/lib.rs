use rand::Rng;
use num::bigint::{ToBigInt, ToBigUint};

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.to_biguint().pow(a.to_biguint()) % p.to_biguint()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}
