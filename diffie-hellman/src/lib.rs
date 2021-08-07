use rand::Rng;
use num::bigint::{ToBigInt, ToBigUint};
use num::{BigUint, ToPrimitive};

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

fn modpow(x: u64, y: u64, z: u64) -> u64 {
    let args:Vec<BigUint> = vec!(x, y, z).into_iter().map(|n| n.to_biguint().unwrap()).collect();
    let (x, y, z) = (args[0].clone(), args[1].clone(), args[2].clone());
    y.modpow(&z, &x).to_u64().unwrap()
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(p, b_pub, a)
}
