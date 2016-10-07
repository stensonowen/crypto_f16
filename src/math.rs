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
    use self::num::traits::{Num, NumCast};
    use std::cmp::PartialOrd;

    use std::mem;

    //Define the type that inflexible/non-generic functions return
    //panic! when this leads to inflexibility
    pub type Output = i32;


    pub fn gcd<T: Num + PartialOrd + Copy>(x: T, y: T) -> T {
        // Euclidean Algorithm
        // generic over all primitive numeric types
        if x.is_zero() {
            y
        } else if y.is_zero() {
            x 
        } else {
            let (a,b) = if x>y { (x,y) } else { (y,x) };
            gcd(b, a%b)
        }
    }

    pub fn coprime<T: Num + PartialOrd + Copy>(x: T, y: T) -> bool { 
        gcd(x,y) == T::one()
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
            r_new = r_new % r_old;  // read: r_old mod r_new

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


}

