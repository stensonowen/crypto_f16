//Elliptic Curve Crypto
//https://en.wikipedia.org/wiki/Elliptic_curve_cryptography


use super::super::math;
use math::Mod;
use num::rational::Rational64;


#[derive(PartialEq)]
pub struct Point {
    //TODO: is float equality going to be a problem?
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point{ x: x, y: y }
    }
}


pub struct ECC {
    //equation of the form `y^2 = x^3 + a*x + b`
    a: f64,
    b: f64,
    p: u64,

}


impl ECC {
    pub fn new(a: f64, b: f64, p: u64) -> ECC {
        //modular exp?
        assert!(4.0 * a.powi(3) + 27.0 * b.powi(2) != 0.0);
        ECC {
            a: a,
            b: b,
            p: p,
        }

    }

    fn contains(&self, p: Point) -> bool {
        p.y.powi(2) == p.x.powi(3) + self.a*p.x + self.b
    }
}


