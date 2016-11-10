//Elliptic Curve Crypto
//https://en.wikipedia.org/wiki/Elliptic_curve_cryptography


use super::super::math;
use math::Mod;


struct ECC {
    //equation of the form `y^2 = x^3 + a*x + b`
    a: f64,
    b: f64,
    p: u64,

}


impl ECC {
    fn new(a: f64, b: f64, p: u64) -> ECC {
        //modular exp?
        assert!(4.0 * a.pow(3) + 27.0 * b.pow(2) != 0);
        ECC {
            a: a,
            b: b,
            p: p,
        }

    }

}

