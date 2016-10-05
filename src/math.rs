// misc math stuff


pub mod math {
    use std::cmp;

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

    pub fn ext_euclidean_alg(a: u32, b: u32) -> (u32, u32) {
        //returns (x,y) such that ax+by = gcd(a,b)
        let(x,y) = (a,b);


        //place holder
        assert!(false);
        (x,y)
    }

}

