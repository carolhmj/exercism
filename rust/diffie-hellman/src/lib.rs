extern crate rand;

use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2,p)    
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}


pub fn mod_exp(b: u64, e: u64, m: u64) -> u64 {
///   The operation of modular exponentiation calculates the 
///   remainder when an integer b (the base) raised to the eth power 
///   (the exponent), be, is divided by a positive integer m (the modulus).

    let mut c : u64 = 1;
    let mut _e : u64 = 0;

    while _e < e {
        _e += 1;
        c = (b * c) % m;
    }

    c    
}