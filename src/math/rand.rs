//random numbers: generators and properties


extern crate num;
use self::num::traits::{Bounded, Unsigned};
use super::Mod;
use std::ops::{BitXor, BitAnd, BitOrAssign, Shl, Shr, ShrAssign};
use std::ops::Add;
use std::num::Wrapping;


//Linear Congruential Generator
//https://en.wikipedia.org/wiki/Linear_congruential_generator
//Generic over nonnegative numbers (of the same type)
//MCG a subset where increment=0

//shortcut to not have to rewrite `Unsigned + Copy` a bunch of times
pub trait LCGReqs : Unsigned + Copy {}
impl<T> LCGReqs for T where T: Unsigned + Copy {}
//impl<T> LCGReqs for T where Wrapping<T> {}

struct GenericWrap<T: LCGReqs>( T );

pub struct LCG<T: LCGReqs> {
    modulus:    GenericWrap<T>,
    multiplier: GenericWrap<T>,
    increment:  GenericWrap<T>,
    seed:       GenericWrap<T>,
}

impl<T: LCGReqs> LCG<T> {
    pub fn from_lcg(modulus: T, multiplier: T, increment: T, seed: T) -> Self {
        //linear congruential generator has multiplier and increment
        LCG {
            modulus:    GenericWrap(modulus),
            multiplier: GenericWrap(multiplier),
            increment:  GenericWrap(increment),
            seed:       GenericWrap(seed),
        }
    }
    pub fn from_mcg(modulus: T, multiplier: T, seed: T) -> Self {
        //multiplicative congruential generator only has multiplier
        Self::from_lcg(modulus, multiplier, T::zero(), seed)
    }
}

impl LCG<u32> {
    pub fn ansi_c() -> Self {
        //ANSI C rand function parameters:
        LCG {
            modulus:    GenericWrap(2u32.pow(31)),
            multiplier: GenericWrap(1103515245),
            increment:  GenericWrap(12345),
            seed:       GenericWrap(12345),
        }
    }
}

impl<T> Add<T> for GenericWrap<T> where T: LCGReqs {
    type Output = GenericWrap<T>;
    fn add(self, rhs: GenericWrap<T>) -> Self::Output {
        rhs
    }
}

impl<T: LCGReqs> Iterator for LCG<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        //generate new value, modify `seed`
        //does not return the initial seed the first time
        //let product = Wrapping(self.multiplier) * Wrapping(self.seed);
        let a = self.multiplier + self.seed;
        //self.seed = (self.multiplier * self.seed + self.increment).modulo(self.modulus);
        Some(self.seed.0)
    }
}


//https://en.wikipedia.org/wiki/Maximum_length_sequence
//for this simple implementation, the length can only be the size of an unsigned integer
//  (in the num crate: u8, u16, u32, u64, (BigUint, usize))

//shortcut to not have to rewrite trait signature. 
//I think this technically uses more characters
pub trait MSeqReqs<T> : Copy + Unsigned + Bounded +
    BitOrAssign<T> + BitAnd<T,Output=T> + BitXor<T,Output=T> +
    ShrAssign<T> + Shl<T,Output=T> + Shr<T,Output=T> {}

impl<S,T> MSeqReqs<T> for S where S: Copy + Unsigned + Bounded + 
    BitOrAssign<T> + BitAnd<T,Output=T> + BitXor<T,Output=T> +
    ShrAssign<T> + Shl<T,Output=T> + Shr<T,Output=T> {}

pub struct MSequence<T: MSeqReqs<T>> {
    state: T,
}

impl<T: MSeqReqs<T>> MSequence<T> {
    pub fn from(block: T) -> Self {
        MSequence {
            state: block,
        }
    }
}

impl<T: MSeqReqs<T>> Iterator for MSequence<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        //shift state right by 1
        //determines new (leftmost) bit by xoring prior rightmost 2 bits
        let two: T = T::one() + T::one();
        let bit0: T = T::one() & self.state;
        let bit1: T = (two & self.state) >> T::one();
        let first_bit = bit0 ^ bit1;
        self.state >>= T::one();
        //want to `OR` self.state with first_bit
        //but T is generic and first_bit is either 0 or 1
        //ordinarily we'd Shl it by 7 or whatever, but we don't know its size
        //instead, if first_bit isn't zero (so it's one), we can find 
        // 0b100..000  from the max value
        if first_bit == T::one() {
            //find largest power of two that fits in T
            //i.e. first bit is 1, all other bits are 0
            let pad: T = T::max_value() - T::max_value() / two;
            self.state |= pad;
        }
        Some(self.state)
    }
}

