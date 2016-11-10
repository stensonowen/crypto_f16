//Elliptic Curve Crypto
//https://en.wikipedia.org/wiki/Elliptic_curve_cryptography


use super::super::math;
use math::Mod;

use num;
//use num::FromPrimitive;
//use num::bigint::BigInt;
//use num::rational::{Ratio, BigRational, Rational64};
//let mut a: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(37).unwrap());


#[derive(PartialEq, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point{ x: x, y: y }
    }
}


pub struct ECC {
    //equation of the form `y^2 = x^3 + a*x + b`
    a: i64,
    b: i64,
    p: i64,

}


impl ECC {
    pub fn new(a: i64, b: i64, p: u64) -> ECC {
        //modular exp?
        let p: i64 = num::cast(p).unwrap();
        assert!(4 * a.mod_exp(3, p) + 27 * b.mod_exp(2, p) != 0);
        ECC {
            a: a,
            b: b,
            p: p,
        }

    }

    pub fn contains(&self, p: &Point) -> bool {
        p.y.mod_exp(2, self.p) == (p.x.mod_exp(3, self.p) + self.a*p.x + self.b).modulo(self.p)
    }

    pub fn slope_between(&self, p1: &Point, p2: &Point) -> i64 {
        //TODO: handle case where "denominator" is zero
        let p: u64 = num::cast(self.p).unwrap();
        let (numer, denom): (i64, i64) = if p1 != p2 {
            (p2.y-p1.y, (p2.x-p1.x).modulo(self.p))
        } else {
            ((3*p1.x.mod_exp(2, self.p) + self.a), 2*p1.y)
        };
        //the value is `numer` / `denom` = `numer` * `denom`^{-1}
        println!("DENOM: {}", denom);
        let multiplicand: i64 = num::cast(math::mult_inverse_signed(denom, p)).unwrap();
        println!("C");
        (numer*multiplicand).modulo(self.p)
    }

    pub fn add(&self, p1: &Point, p2: &Point) -> Point {
        let m = self.slope_between(p1, p2);
        let x = (m.mod_exp(2, self.p) - p1.x - p2.x).modulo(self.p);
        let y = (m*(p1.x - x) - p1.y).modulo(self.p);

        Point::new(x,y)
    }
}


