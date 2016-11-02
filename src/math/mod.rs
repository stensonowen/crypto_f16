// misc math stuff

/* Try to use generics as much as possible;
 *  However, generic functions can only return a generic type
 *  if the output is strictly less than (and similarly signed)
 *  compared to the inputs.
 *      Problematic examples: 100u8 * 100u8, 2u32 - 5u32, etc.
 *  Otherwise return i64s? i32s?
 */

//mod primes;       //?
pub mod rand;

//Number traits
extern crate num;
use self::num::traits::{Num, NumCast, Unsigned, /*Bounded,*/ /*CheckedMul*/};
use std::cmp::PartialOrd;
use std::ops::{Add, Rem};


//Define the type that inflexible/non-generic functions return
//panic! when this leads to inflexibility
pub type Output = i32;


//Define modulus operator on all numbers
//deals with negative numbers correctly
// a mod n  ↔  a.modulo(n)
//  `a` and `n` must be the same type
//  type must support Copy and Rem/Add operators
//Apparently it can be bad practice to implement traits on primitives.
// However I'm not going to change this because it is well defined 
//  for all numeric types (and because it arguably should be in the 
//  language in the first place)
pub trait Mod<T> {
    fn modulo(self, n: T) -> T ;
    fn exp(self, e: u32, n: T) -> T;
}
impl<T> Mod<T> for T where T: Rem<Output=T> + Add<Output=T> + Copy {
    fn modulo(self, n: T) -> T {
        ((self%n)+n)%n
    }
    fn exp(self, e: u32, n: T) -> T {
        // a.exp(b, c) = (a ** b) mod c
        // exponent must be unsigned
        //strat: (a*b) mod n ≡ (a mod n)*(b mod n)
        //  calculating the mod at every iteration is O(n) but safe
        //  calculating the mod at the end is O(1) but would probably overflow
        //  square the appropriate number of times, calculating the mod each time
        //      safe as long as n ≤ √T::MAX
        //      TODO: verify this?

        self
    }
}


//pub fn abs<T: Num + PartialOrd + CheckedMul>(a: T) -> Option<T> {
/* Complicated problem: Do we require CheckedMul for everything that calls this
 *  or require Bounded or something? Or verify somehow this is correct?
 *  Or just assume it's fine?
 *  OR create `modular_abs(a,p)` that returns the correct result WITHOUT overflow?
 */
pub fn abs<T: Num + PartialOrd>(a: T) -> T {
    if a >= T::zero() {
        a
    } else {
        //a.checked_mul(&(T::zero() - T::one())).unwrap()
        a * (T::zero() - T::one())
    }
}


pub fn gcd<T: Num + PartialOrd + Copy>(mut x: T, mut y: T) -> T {
    // Euclidean Algorithm
    // generic over all primitive numeric types
    //let a = x * (T::zero() - T::one());
    //if inputs are negative, take the absolute value
    x = abs(x);
    y = abs(y);

    if x.is_zero() {
        y
    } else if y.is_zero() {
        x 
    } else {
        let (a,b) = if x>y { (x,y) } else { (y,x) };
        //gcd(b, a%b)
        gcd(b, a.modulo(b))
    }
}

pub fn coprime<T: Num + PartialOrd + Copy>(x: T, y: T) -> bool { 
    gcd(x,y) == T::one()
}

pub fn mult_inverse_signed<S: NumCast+Copy+PartialOrd+Num, 
                  T: Unsigned+NumCast+Copy+PartialOrd+Num>(a: S, n: T) -> Output {
    //find some b=1/a such that a*b ≡ 1 (mod n)
    //always returns a negative number b
    //as such, the result requires that modulus is done with `Mod` trait and NOT `%`
    //  otherwise a negative times a positive is always negative,
    //  and a negative `%` a positive is also negative, 
    //  and 1 is not negative.
    
    //to operate on `a` and `n` they must be of the same type
    //cannot cast both to `S` or `T` because `a` might be <0 and `n` might be too big
    let a_o: Output = NumCast::from(a).unwrap();
    let n_o: Output = NumCast::from(n).unwrap();
    assert!(coprime(a_o,n_o));
    //now we can find some positive `a` for which we can use `mult_inverse`
    
    //this cast will always succeed because 0 ≤ `a mod n` < n
    let a_t: T = NumCast::from(a_o.modulo(n_o)).unwrap();
    mult_inverse(a_t, n)
}

pub fn mult_inverse<T: NumCast+Unsigned+Copy+PartialOrd>(a: T, n: T) -> Output {
    //find some b=1/a such that a*b ≡ 1 (mod n)
    assert!(coprime(a,n));  //inputs must be coprime
    //perform extended euclidean algorithm
    let (x,_) = ext_euclidean_alg(a,n);
    // now x*a + y*p = 1
    // thus x*a (mod n) = 1 or -1
    if x >= 0 { 
        // then x*a is positive and y*p is negative
        // so x*a (mod n) ≡ 1
        x 
    } else { 
        // then x*a is negative and y*p is positive
        // so x*a (mod n) < 0; wrap around by adding `n`
        //make sure we can cast modulus to the Output type
        let n: Output = NumCast::from(n).unwrap();
        n+x
    }
}

pub fn ext_euclidean_alg<T: NumCast>(a: T, b: T) -> (Output, Output) {
    //returns (x,y) such that ax+by = gcd(a,b)
    //return gcd as well (i.e. 3-tuple)? 
    use std::mem;

    let a: Output = NumCast::from(a).unwrap();  //convert input to i32 vals
    let b: Output = NumCast::from(b).unwrap();  //panic if they're too big
    let in_order = a > b;   //used to determine which is x vs y
    let (mut x_old, mut x_new) = (1, 0);
    let (mut y_old, mut y_new) = (0, 1);
    let (mut r_old, mut r_new) = if in_order { (a,b) } else { (b,a) };
    let mut q: i32;

    //println!("\nr_i,\tq_i,\tx_i,\ty_i");
    //println!("{},\t{},\t{},\t{}", r_old, '_', x_old, y_old);
    //println!("{},\t{},\t{},\t{}", r_new, '_', x_new, y_new);

    loop {
        //r_old gets removed, r_new replaces it, calculate new r_new
        mem::swap(&mut r_old, &mut r_new);
        q = r_new / r_old;      // read: r_old div r_new
        //r_new = r_new % r_old;  // read: r_old mod r_new
        r_new = r_new.modulo(r_old);  // read: r_old mod r_new

        if r_new == 0 { break }

        //x_old removed, x_new replaces x_old, new x_new
        mem::swap(&mut x_old, &mut x_new);
        x_new = x_new - x_old * q;
        mem::swap(&mut y_old, &mut y_new);
        y_new = y_new - y_old * q;

        //println!("{},\t{},\t{},\t{}", r_new, q, x_new, y_new);
    }

    if in_order { (x_new, y_new) } else { (y_new, x_new) }
}

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

//fn entrope()


