//#![feature(test)]     //benchmark using nightly

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
    fn gcd() {
        assert_eq!( 1, math::gcd(14u16, 15u16));
        assert_eq!( 7, math::gcd(14i64, 21i64));
        assert_eq!(13, math::gcd(13u8, 26u8));
        assert_eq!( 1, math::gcd(42i32, 7919i32));
        assert_eq!( 1, math::gcd(61157u32, 32414u32));
        assert_eq!(42, math::gcd((42*61157) as u64, (42*32414) as u64));
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



    //  MISC BENCHMARKS:

    /*
    extern crate test;
    use self::test::Bencher;
    */

    /* Testing: performance for different primitive types
     *  Conclusion:
     *      unsigned seem to be a faster than signed (particularly 64-bit)
     *      32-bit quite a bit faster than 64-bit
     *      16-bit between them (unsigned, anyway)
     *      u32 ≅ i32
     *      i64 100% slower than _32
     *      u64 ~60% slower than _32
     *  Prescription:
     *      Try to avoid i64s
     *      Signed results should probably be i32s not i64s
     */
    /*
    #[bench]
    fn bench_gcd_u32(b: &mut Bencher) { 
        b.iter(|| math::gcd(61157u32, 32414u32)) 
    }
    #[bench]
    fn bench_gcd_i32(b: &mut Bencher) { 
        b.iter(|| math::gcd(61157i32, 32414i32)) 
    }
    #[bench]
    fn bench_gcd_u64(b: &mut Bencher) { 
        b.iter(|| math::gcd(61157u64, 32414u64)) 
    }
    #[bench]
    fn bench_gcd_i64(b: &mut Bencher) { 
        b.iter(|| math::gcd(61157i64, 32414i64)) 
    }
    */
    /* Test different methods of swapping values
     * (After being rebuilt on nightly, presumably w/ optimizations):
     *  All took 17 ns
     */
    /*
    use std::mem;
    #[bench]
    fn swap_with_swap(b: &mut Bencher) { 
        let (mut x, mut y) = (61157i64, 32414i64); 
        b.iter(|| { for _ in 0..1000 { mem::swap(&mut x, &mut y); } })
    }

    #[bench]
    fn swap_with_tmp(b: &mut Bencher) { 
        let (mut x, mut y) = (61157i64, 32414i64); 
        b.iter(|| { for _ in 0..1000 { let tmp = x; x = y; y = tmp; } })
    }

    #[bench]
    fn swap_with_magic(b: &mut Bencher) { 
        let (mut x, mut y) = (61157i64, 32414i64); 
        b.iter(|| { for _ in 0..1000 { x ^= y; y ^= x; x ^= y; } })
    }
    */
}


