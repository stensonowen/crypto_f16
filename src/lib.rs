//#![feature(test)]     //benchmark using nightly

pub mod math;
//pub mod crypto;
extern crate num;



#[cfg(test)]
mod tests {
    //math module
    use math;
    //number traits
    use math::Mod;
    use super::num::traits::NumCast;


    //MODULAR ARITHMETIC tests

    // Test the `modulo` operator
    #[test]
    fn mod_signed() {
        use math::Mod;
        assert_eq!(5%2, 1);
        assert_eq!(-5%2, -1);
        let a: i32 = -5;
        assert_eq!(a.modulo(2), 1);
        assert_eq!((-5).modulo(2), 1);
        //vector of tests: v.i.0 mod v.i.1 = v.i.2
        let v: Vec<(i32,i32,i32)> = vec![(-98, 308, 210), (-443, 363, 283), (-413, 443, 30), 
                                        (-96, 59, 22),    (-268, 24, 20),   (-449, 414, 379), 
                                        (-176, 45, 4),    (-469, 413, 357), (-486, 40, 34), 
                                        (-359, 126, 19),  (-270, 138, 6),   (-107, 430, 323), 
                                        (-112, 340, 228), (-411, 310, 209), (-304, 398, 94), 
                                        (-60, 60, 0),     (-200, 446, 246), (-82, 495, 413), 
                                        (-132, 277, 145), (-2, 139, 137),   (-465, 61, 23), 
                                        (-209, 122, 35),  (-438, 393, 348), (-437, 31, 28), 
                                        (-316, 301, 286), (-26, 97, 71),    (-16, 156, 140), 
                                        (-127, 193, 66),  (-446, 170, 64),  (-404, 216, 28) ];
        for (a,b,c) in v {
            assert_eq!(a.modulo(b), c);
        }
    }

    //test greatest common denominator
    #[test]
    fn gcd() {
        //base case tests
        assert_eq!(0, math::gcd(0,0));
        assert_eq!(4, math::gcd(4,0));
        assert_eq!(4, math::gcd(0,4));
        //actual tests
        assert_eq!( 1, math::gcd(14u16, 15u16));
        assert_eq!( 7, math::gcd(14i64, 21i64));
        assert_eq!(13, math::gcd(13u8, 26u8));
        assert_eq!( 1, math::gcd(42i32, 7919i32));
        assert_eq!( 1, math::gcd(61157u32, 32414u32));
        assert_eq!(42, math::gcd((42*61157) as u64, (42*32414) as u64));
        //negatives
        assert_eq!( 1, math::gcd( 14, -15));
        assert_eq!( 4, math::gcd(-36,  20));
        assert_eq!( 1, math::gcd(-25, -33));    //correct? or -1?
        assert_eq!( 3, math::gcd(-21,  48));
        assert_eq!( 1, math::gcd(-11, -20));
        assert_eq!( 1, math::gcd(  9, -13));
        assert_eq!(43, math::gcd( 43, -43));
        assert_eq!(13, math::gcd(-13,  26));
        assert_eq!( 1, math::gcd(-27,  41));
        assert_eq!( 4, math::gcd( 36, -16));
    }

    //Extended Euclidean Algorithm
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

        //u64s sometimes don't fit into i32s, but they can (only panic if overflow)
        let (a,b): (u64,u64) = (67571, 42578);
        let (x,y) = math::ext_euclidean_alg(a, b);
        assert_eq!(x*(a as i32) + y*(b as i32), 1);

        let (a,b) = (1234, 4321);
        let (x,y) = math::ext_euclidean_alg(a, b);
        assert_eq!(x*a + y*b, 1);
    }

    //Make sure NumCasts::from().unwrap() catches numbers that are too large
    #[test]
    #[should_panic]
    fn ext_euclid_alg_exceed() {
        //can't cast a large u64 down to an i32 
        let (a,b): (u64,u64) = (1099511627776, 549755813888);   // 2**40, 2**39 
        math::ext_euclidean_alg(a, b);
    }


    //Modular inverse
    #[test]
    fn modular_inverse_unsigned() {
        //be super sure
        //input tests: random coprime pairs 
        let v: Vec<(u32,u32)> = vec![(5,24),    (121, 133), (97,31),    (100, 199), (24,49), 
                                    (35, 761),  (723, 997), (927, 718), (153, 566), (256, 27), 
                                    (394, 805), (173, 386), (768, 391), (399, 59),  (14, 885), 
                                    (385, 509), (357, 143), (272, 617), (509, 847), (218, 461), 
                                    (718, 413), (725, 482), (298, 223), (533, 250), (478, 383), 
                                    (863, 641), (931, 324), (221, 324), (824, 537), (409, 203), 
                                    (526, 231), (452, 601), (688, 547), (721, 274), (722, 629), 
                                    (169, 165), (735, 703), (843, 407), (581, 876), (909, 989)];
        for (a,p) in v {
            let y = math::mult_inverse(a,p);
            assert!(y > 0);
            let x: u32 = NumCast::from(y).unwrap();
            assert_eq!((a*x)%p, 1);
            assert_eq!((a*x).modulo(p), 1);
        }
    }

    #[test]
    fn modular_inverse_signed() {
        let v: Vec<(i32,u32)> = vec![(-219, 524), (499, 650), (817, 333),  (-913, 30), 
                                    (-973, 746),  (143, 128), (-995, 497), (-18, 521), 
                                    (-6, 235),    (769, 573), (-801, 614), (-764, 285), 
                                    (-373, 297),  (952, 815), (699, 784),  (255, 922)];

        for (a,p) in v {
            let y = math::mult_inverse_signed(a,p);
            assert!(y > 0);
            let x: i32 = NumCast::from(y).unwrap();
            assert_eq!((a*x).modulo(p as i32), 1);
        }
    }


    //PRIME tests
    
    //Find the prime factors of a number
    //Shouldn't actually use this because it scales poorly
    #[test]
    #[ignore]   //big numbers: can be slow (~10 seconds)
    fn prime_factorize() {
        fn is_prime_naive(n: u32) -> bool {
            n > 1 && !(2 .. 1 + n/2).into_iter().any(|i| n%i==0) 
        }
        fn reform_from_prime_factors(f: Vec<(u32,u32)>) -> u32 {
            f.into_iter().fold(1u32, |acc, (n,e)| acc*n.pow(e))
        }

        let vals = vec![22176180, 137235605, 912673, 47];
        for i in vals {
            let v = math::prime_factors(i);
            println!("\n{:?}", v);
            for &(j,_) in &v {
                assert!(is_prime_naive(j));
            }
            assert_eq!(reform_from_prime_factors(v), i);
        }
    }


    //RAND tests
    
    //test basic LCM
    #[test]
    fn demo_lcg() {
        use math::rand;
        //demos taken from https://en.wikipedia.org/wiki/Linear_congruential_generator 
        //https://en.wikipedia.org/wiki/File:Linear_congruential_generator_visualisation.svg
        //let lcg = rand::LCG::from_lcg(9u32, 2u32, 0u32, 1u32);
        let lcg = rand::LCG::from_mcg(9u32, 2u32, 1u32);
        let expected = vec![2, 4, 8, 7, 5, 1];
        let actual: Vec<u32> = lcg.into_iter().take(expected.len()).collect();
        assert_eq!(expected, actual);

        let lcg: rand::LCG<u32> = rand::LCG::from_lcg(9, 4, 1, 0);
        let expected = vec![1, 5, 3, 4, 8, 6, 7, 2, 0];
        let actual: Vec<u32> = lcg.into_iter().take(expected.len()).collect();
        assert_eq!(expected, actual);
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


