/*
 * Basic DES implementation in Rust
 */

use math::math;

#[allow(dead_code, unused_variables)]
pub fn des(ptxt: u64, key: u64) -> u64 {
    // encrypts 64 bits of plaintext at a time
    // uses 56 bits of a 64-bit key
    0u64
}


#[allow(dead_code, unused_variables)]
pub fn subkey(mut key: u16, i: u8) -> u16 {
    //key = 9 bits
    //get 8 wrapping bits i bits after the most significant
    assert!(key <= 0b0000000111111111);     //9  bits
    //move all bits to the leftmost position
    key << 7;
    
    /*  i=1: >>1
     *  i=2: 
     *
     */

    // (123456789, 1) => 12345678
    // (123456789, 2) => 23456789
    // (123456789, 3) => 34567891
    // (123456789, 4) => 45678912
    // (123456789, 5) => 
    // (123456789, 6) => 
    // (123456789, 7) => 
    // (123456789, 8) => 
    // (123456789, 9) => 


    

    0
}

#[allow(dead_code, unused_variables)]
pub fn sdes(ptxt: u16, key: u16) -> u16 {
    //simplified des
    // ptxt = ctxt = 12 bits
    // key = 9 bits, 8 at a time
    
    //verify the ranges are correct
    assert!(ptxt <= 0b0000111111111111);    //12 bits
    assert!(key  <= 0b0000000111111111);    //9  bits
    
    0u16

}

