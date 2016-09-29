// misc math stuff


pub mod math {
    use std::cmp;

    pub fn gcd(x: u32, y: u32) -> u32 {
        // euler's algorithm
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

}

