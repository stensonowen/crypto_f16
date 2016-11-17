//compute Jacobi symbol of a number

use super::Mod;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Jacobi {
    Value(i8),  // -1, 0, 1
    Frac{pos: bool, numer: u64, denom: u64},
}

impl Jacobi {
    fn from_val(a: i8) -> Jacobi {
        Jacobi::Value(a)
    }
    fn from_frac(s: bool, a: u64, b: u64) -> Jacobi {
        Jacobi::Frac{pos: s, numer: a, denom: b}
    }
    fn print(&self) {
        if let &Jacobi::Frac{pos: p, numer: n, denom: d} = self {
            println!(" {} J( {} / {} )", 
                     if p { '+' } else { '-' }, n, d);
        } else if let &Jacobi::Value(v) = self {
            println!(" {} ", v)
        }
    }

    fn reduce(&self) -> Jacobi {
        //let reductions = vec![Jacobi::modulo, Jacobi::two_numer, Jacobi::reciprocal];
        let a = &Jacobi::modulo;
        let b = &Jacobi::two_numer;
        let c = &Jacobi::reciprocal;;
        let mut reductions: Vec<&Fn(&Jacobi) -> Option<Jacobi>> = Vec::new();
        reductions.push(a);
        reductions.push(b);
        reductions.push(c);

        let mut attempts: HashSet<Jacobi> = HashSet::new();

        loop {

        }
        Jacobi::Value(0)
    }


    //methods of reducing:
    fn modulo(&self) -> Option<Jacobi> {
        // if a≡b(mod p), L(a/p) = L(b/p)
        if let &Jacobi::Frac{pos: p, numer: n, denom: d} = self {
            Some(Jacobi::from_frac(p, n.modulo(d), d))
        } else {
            None
        }
    }
    fn two_numer(&self) -> Option<Jacobi> {
        // J((ab)/n) = J(a/n)*J(b/n)
        // J(2/n) = { 1 iff n≡±1(mod 8), -1 iff n≡ٍ±3(mod 8) }
        if let &Jacobi::Frac{pos: p, numer: n, denom: d} = self {
            let z = n.trailing_zeros();
            let n_ = n / 2u64.pow(z);
            let rem = d.modulo(8);
            let sign: i8 = {
                if rem == 1 || rem == 7 { 1 } 
                else if rem == 3 || rem == 5 { -1 } 
                else { return None }
            };
            let mut acc_sign = sign.pow(z);
            if !p { acc_sign *= -1; }
            Some(Jacobi::from_frac(acc_sign == 1, n, d))
        } else {
            None
        }
    }
    fn reciprocal(&self) -> Option<Jacobi> {
        // J(m/n) = {-J(n/m) if m≡n≡3(mod 4)
        if let &Jacobi::Frac{pos: p, numer: n, denom: d} = self {
            if n.modulo(4) == 3 && d.modulo(4) == 3 {
                Some(Jacobi::from_frac(!p, d, n))
            } else {
                None
            } 
        } else {
            None
        }
    }

}
