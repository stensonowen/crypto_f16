pub mod math;

#[cfg(test)]
mod tests {
    use super::math::math;

    #[test]
    #[ignore]
    fn it_works() {
    }

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
}


