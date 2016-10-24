//random number generate


extern crate num;
use self::num::traits::Unsigned;
use super::Mod;

pub struct LCG<T: Unsigned + Copy> {
    //https://en.wikipedia.org/wiki/Linear_congruential_generator
    //MCPG a subset: increment=0
    //Generic, but all values must be nonnegative
    modulus:    T,
    multiplier: T,
    increment:  T,
    seed:       T,
}

impl<T: Unsigned + Copy> LCG<T> {
    //              m           a              c             X_0
    pub fn from_lcg(modulus: T, multiplier: T, increment: T, seed: T) -> Self {
        LCG {
            modulus:    modulus,
            multiplier: multiplier,
            increment:  increment,
            seed:       seed,
        }
    }
    pub fn from_mcg(modulus: T, multiplier: T, seed: T) -> Self {
        LCG {
            modulus:    modulus,
            multiplier: multiplier,
            increment:  T::zero(),
            seed:       seed,
        }
    }
}

impl<T: Unsigned + Copy> Iterator for LCG<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        //generate new value, modify `seed`
        //does not return the initial seed the first time
        self.seed = (self.multiplier * self.seed + self.increment).modulo(self.modulus);
        Some(self.seed)
    }
}


