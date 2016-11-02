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



