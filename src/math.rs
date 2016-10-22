// misc math stuff

/* Try to use generics as much as possible;
 *  However, generic functions can only return a generic type
 *  if the output is strictly less than (and similarly signed)
 *  compared to the inputs.
 *      Problematic examples: 100u8 * 100u8, 2u32 - 5u32, etc.
 *  Otherwise return i64s? i32s?
 */



pub mod math {
    extern crate num;
    use self::num::traits::{Num, NumCast, Unsigned, /*CheckedMul*/};
    use std::cmp::PartialOrd;
    use std::ops::{Add, Rem};

    use std::mem;

    //Define the type that inflexible/non-generic functions return
    //panic! when this leads to inflexibility
    pub type Output = i32;


    //Define modulus operator on all numbers
    //deals with negative numbers correctly
    // a mod n  ↔  a.modulo(n)
    //  `a` and `n` must be the same type
    //  type must support Copy and Rem/Add operators
    // What should the function be called?
    //  `mod` is ideal but is a reserved keyword
    //  `modulo` is kind of long
    //  `mod_`? `m0d`??
    pub trait Mod<T> {
        fn mod_(self, n: T) -> T ;
    }
    impl<T> Mod<T> for T where T: Rem<Output=T> + Add<Output=T> + Copy {
        fn mod_(self, n: T) -> T {
            ((self%n)+n)%n
        }
    }

    //misc math functions that don't exist or aren't generic
    //pub fn modulo<T: Num+Copy>(a: T, b: T) -> T {
    //    ((a%b)+b)%b
    //}
    //pub fn abs<T: Num + PartialOrd + CheckedMul>(a: T) -> Option<T> {
    //pub fn abs<T: Num + PartialOrd + Neg<Output=T>>(a: T) -> Option<T> {
    pub fn abs<T: Num + PartialOrd>(a: T) -> T {
        if a >= T::zero() {
            a
        } else {
            //a.checked_mul(&(T::zero() - T::one())).unwrap()
            //-a
            a * (T::zero() - T::one())
        }
    }


    pub fn gcd<T: Num + PartialOrd + Copy>(mut x: T, mut y: T) -> T {
        // Euclidean Algorithm
        // generic over all primitive numeric types
        //println!("gcd( {:?} , {:?} )", x, y);
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
            //gcd(b, modulo(a,b))
            gcd(b, a.mod_(b))
        }
    }

    pub fn coprime<T: Num + PartialOrd + Copy>(x: T, y: T) -> bool { 
        gcd(x,y) == T::one()
    }

    //pub fn mult_inverse_signed<S: NumCast, T: NumCast+Unsigned>(a: S, n: T) -> Output {
        //modulus must be positive
        //for some of our logic we're going to assume `a` is also positive:
        //
        //make `a` positive type T
    pub fn mult_inverse_signed<S: NumCast+Copy+PartialOrd+Num, 
                      T: Unsigned+NumCast+Copy+PartialOrd+Num>(a: S, n: T) -> Output {
        //should modulo be unsigned?
        //  Y:  + can check sign at compile time
        //      - must convert `a` to corresponding positive value first
        //  N:  + easier? just assert it's positive and then call eea() normally?
        //      - must cast everything to `Output` first? (ext_euclid requires same type)
        let a_: Output = NumCast::from(a).unwrap();
        let n_: Output = NumCast::from(n).unwrap();
        let a_2 = a_.mod_(n_);
        //let a: T = NumCast::from(a_.mod_(n_)).unwrap();
        assert!(coprime(a_2,n_));
        let (x,_) = ext_euclidean_alg(a_2, n_);
        x
    }

    //pub fn mult_inverse<T: NumCast+Unsigned>(a: T, n: T) -> Output {
    //TODO: should this be able to return a negative??
    pub fn mult_inverse<T: NumCast+Unsigned+Copy+PartialOrd>(a: T, n: T) -> Output {
        //find some b=1/a such that a*b ≡ 1 (mod n)
        //inputs must be coprime
        assert!(coprime(a,n));
        //perform extended euclidean algorithm
        // x*a + y*p = 1
        let (x,_) = ext_euclidean_alg(a,n);
        x
    }
    /*
    pub fn mult_inverse(a: i32, n: i32) -> i32 {
        //find b=a^-1 such that b*a ≡ 1 mod n
        //place holder
        assert!(coprime(a,n));
        assert!(false);
        0
        //let (b, _) = ext_euclidean_alg(a, n);
        //b
        //this shouldn't work because we don't know if |ax|>|by| or |ax|<|by|
        // pretty sure this could yield 1/a such that a*1/a = (n-1) mod n
    }*/

    pub fn ext_euclidean_alg<T: NumCast>(a: T, b: T) -> (Output, Output) {
        //returns (x,y) such that ax+by = gcd(a,b)
        //return gcd as well (i.e. 3-tuple)? 

        let a: Output = NumCast::from(a).unwrap();  //convert input to i32 vals
        let b: Output = NumCast::from(b).unwrap();  //panic if they're too big
        let in_order = a > b;   //used to determine which is x vs y
        let (mut x_old, mut x_new) = (1, 0);
        let (mut y_old, mut y_new) = (0, 1);
        let (mut r_old, mut r_new) = if in_order { (a,b) } else { (b,a) };
        let mut q: i32;

        println!("\nr_i,\tq_i,\tx_i,\ty_i");
        println!("{},\t{},\t{},\t{}", r_old, '_', x_old, y_old);
        println!("{},\t{},\t{},\t{}", r_new, '_', x_new, y_new);

        loop {
            //r_old gets removed, r_new replaces it, calculate new r_new
            mem::swap(&mut r_old, &mut r_new);
            q = r_new / r_old;      // read: r_old div r_new
            //r_new = r_new % r_old;  // read: r_old mod r_new
            //r_new = modulo(r_new, r_old);  // read: r_old mod r_new
            r_new = r_new.mod_(r_old);  // read: r_old mod r_new

            if r_new == 0 { break }

            //x_old removed, x_new replaces x_old, new x_new
            mem::swap(&mut x_old, &mut x_new);
            x_new = x_new - x_old * q;
            mem::swap(&mut y_old, &mut y_new);
            y_new = y_new - y_old * q;

            println!("{},\t{},\t{},\t{}", r_new, q, x_new, y_new);
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
            //if sieve.get(i as usize) == Some(&false) && modulo(n,i) == 0 {
            if sieve.get(i as usize) == Some(&false) && n.mod_(i) == 0 {
                //`i` is prime and divides what's left of `n`
                let mut exp: u32 = 0;
                loop {
                    n /= i;
                    exp += 1;
                    //if n%i != 0 { break }
                    //if modulo(n,i) != 0 { break }
                    if n.mod_(i) != 0 { break }
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

}

