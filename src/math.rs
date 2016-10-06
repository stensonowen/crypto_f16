// misc math stuff


pub mod math {
    use std::cmp;
    use std::mem;

    pub fn gcd(x: u32, y: u32) -> u32 {
        // Euclidean Algorithm
        // only accept nonnegative numbers
        if x == 0 {
            y
        } else if y == 0 {
            x 
        } else {
            let (a, b) = (cmp::max(x,y), cmp::min(x,y));
            gcd(b, a%b)
        }
    }

    pub fn coprime(x: u32, y: u32) -> bool {
        gcd(x,y) == 1
    }

    pub fn mult_inverse(a: u32, n: u32) -> u32 {
        //find b=a^-1 such that b*a ≡ 1 mod n
        assert!(coprime(a,n));


        //place holder
        assert!(false);
        0
    }

    pub fn ext_euclidean_alg(a: i32, b: i32) -> (i32, i32) {
        //returns (x,y) such that ax+by = gcd(a,b)
        //return gcd as well (i.e. 3-tuple)? 
        //stallings p99
        let in_order = a > b;   //used to determine which is x vs y
        let (mut x_old, mut x_new) = (1, 0);
        let (mut y_old, mut y_new) = (0, 1);
        //let (mut r_old, mut r_new) = (cmp::max(a,b), cmp::min(a,b));
        let (mut r_old, mut r_new) = if in_order { (a,b) } else { (b,a) };
        let mut q: i32;

        println!("");
        println!("r_i,\tq_i,\tx_i,\ty_i");
        println!("{},\t{},\t{},\t{}", r_old, '_', x_old, y_old);
        println!("{},\t{},\t{},\t{}", r_new, '_', x_new, y_new);

        while r_new != 0 {
            //r_old gets removed, r_new replaces it, calculate new r_new
            mem::swap(&mut r_old, &mut r_new);
            q = r_new / r_old;      // read: r_old div r_new
            r_new = r_new % r_old;  // read: r_old mod r_new

            //replace `while` loop with this?
            //a little clunky but technically faster?
            //if r_new == 0 { return (x_new, y_new) }

            //x_old removed, x_new replaces x_old, new x_new
            mem::swap(&mut x_old, &mut x_new);
            x_new = x_new - x_old * q;
            //same for y
            mem::swap(&mut y_old, &mut y_new);
            y_new = y_new - y_old * q;

            println!("{},\t{},\t{},\t{}", r_new, q, x_new, y_new);
        }

        //(x_new, y_new)
        //(x_old, y_old)
        if in_order { (x_old, y_old) } else { (y_old, x_old) }

    }

}

