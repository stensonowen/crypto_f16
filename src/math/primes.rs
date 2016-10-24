// misc primes stuff
// primality testing / factoring?

use super::Mod;

#[allow(dead_code)]
pub fn prime_factors(mut n: u32) -> Vec<(u32,u32)> {
    //use sieve of eratosthenes to compute primes less than n/2
    //return tuples of the prime and the number of times it divides n
    // n = f.into_iter().fold(1u32, |acc, (n,e)| acc*n.pow(e))
    use std::io::{self, Write}; //write to stderr
    io::stderr().write(b"Warning: `prime_factors()` is not an efficient function \
                       and should only be used on small numbers\n").unwrap();
    let max = n+1;
    let mut sieve = Vec::<bool>::with_capacity(max as usize);    // true ↔ composite
    sieve.resize(max as usize, false);
    let mut factors = Vec::<(u32,u32)>::new();
    for i in 2_u32 .. max {
        //if sieve.get(i as usize) == Some(&false) && n%i == 0 {
        if sieve.get(i as usize) == Some(&false) && n.modulo(i) == 0 {
            //`i` is prime and divides what's left of `n`
            let mut exp: u32 = 0;
            loop {
                n /= i;
                exp += 1;
                //if n%i != 0 { break }
                if n.modulo(i) != 0 { break }
            }
            factors.push((i, exp));
            if n == 1 { break }
            for j in 2*i .. max/i {
                sieve[(i*j) as usize] = true;   //no multiples will be prime
            }
        }
    }
    factors
}

#[allow(dead_code)]
pub fn totatives(_: u32) -> Vec<u32> {
    //find totatives of a number using sieve of eratosthenes
    //input must be positive
    //results must be positive and ≤ input
    //let primes = vec![];
    //let possibilities = Vec::<bool>::with_capacity(n as usize);
    //find all the primes less than or equal to `n`
    //for t in 2..(n/2) {
        //2 is the smallest prime, and 
    //}
    //primes
    assert!(false);
    vec![]
}


