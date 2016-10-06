pub mod math;

#[cfg(test)]
mod tests {
    use super::math::math;

    #[test]
    fn gcd_base() {
        assert_eq!(0, math::gcd(0,0));
        assert_eq!(4, math::gcd(4,0));
        assert_eq!(4, math::gcd(0,4));
    }

    #[test]
    fn gcd_common() {
        assert_eq!( 1, math::gcd(14, 15));
        assert_eq!( 7, math::gcd(14, 21));
        assert_eq!(13, math::gcd(13, 26));
        assert_eq!( 1, math::gcd(42, 7919));
        assert_eq!( 1, math::gcd(61157, 32414));
        assert_eq!(42, math::gcd(42*61157, 42*32414));
    }

    #[test]
    fn gcd_generic() {
        assert_eq!( 1, math::gcd_generic(14u16, 15u16));
        assert_eq!( 7, math::gcd_generic(14i64, 21i64));
        assert_eq!(13, math::gcd_generic(13u8, 26u8));
        assert_eq!( 1, math::gcd_generic(42i32, 7919i32));
        assert_eq!( 1, math::gcd_generic(61157u32, 32414u32));
        assert_eq!(42, math::gcd_generic((42*61157) as u64, (42*32414) as u64));
    }


    #[test]
    fn ext_euclid_alg() {
        //common case: stallings p99
        let (a,b) = (1759, 550);
        let (x,y) = math::ext_euclidean_alg(a, b);
        assert_eq!(x*a + y*b, 1);

        //out of order, gcd != 1:
        //https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Example
        let (a,b) = (46, 240);
        let (x,y) = math::ext_euclidean_alg(a, b);
        assert_eq!(x*a + y*b, 2);

        let (a,b) = (1234, 4321);
        let (x,y) = math::ext_euclidean_alg(a, b);
        assert_eq!(x*a + y*b, 1);
    }

}


