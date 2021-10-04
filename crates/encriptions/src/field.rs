use num::{BigInt, BigUint, Integer, One, Signed};
use num_bigint::{Sign, ToBigInt};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

trait Prime {
    fn get_prime() -> BigUint;
}

macro_rules! def_prime_struct {
    ($name: ident, $value: literal) => {
        #[derive(Debug, PartialEq)]
        struct $name;

        impl Prime for $name {
            fn get_prime() -> BigUint {
                BigUint::from($value as u64)
            }
        }
    };
}

def_prime_struct!(Prime13, 13);
def_prime_struct!(Prime19, 19);
def_prime_struct!(Prime29, 29);

#[derive(Debug, PartialEq)]
struct FieldElement<P: Prime>(BigUint, PhantomData<P>);

impl<P: Prime> FieldElement<P> {
    pub fn new(value: BigUint) -> Option<Self> {
        if value >= P::get_prime() {
            None
        } else {
            Some(Self(value, PhantomData))
        }
    }

    pub fn new_from_u64(value: u64) -> Option<Self> {
        Self::new(BigUint::from(value))
    }

    pub fn pow(&self, exponent: &BigInt) -> Self {
        let exponent = rem_euclid(exponent, &(P::get_prime() - BigUint::one()));
        Self(self.0.modpow(&exponent, &P::get_prime()), PhantomData)
    }
}

impl<P: Prime> Add for &FieldElement<P> {
    type Output = FieldElement<P>;

    fn add(self, rhs: Self) -> Self::Output {
        FieldElement((&self.0 + &rhs.0) % P::get_prime(), PhantomData)
    }
}

impl<P: Prime> Sub for &FieldElement<P> {
    type Output = FieldElement<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        FieldElement((&self.0 + (-rhs).0) % P::get_prime(), PhantomData)
    }
}

impl<P: Prime> Mul for &FieldElement<P> {
    type Output = FieldElement<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        FieldElement((&self.0 * &rhs.0) % P::get_prime(), PhantomData)
    }
}

impl<P: Prime> Neg for &FieldElement<P> {
    type Output = FieldElement<P>;

    fn neg(self) -> Self::Output {
        FieldElement(
            rem_euclid(&(-(self.0.to_bigint().unwrap())), &P::get_prime()),
            PhantomData,
        )
    }
}

impl<P: Prime> Div for &FieldElement<P> {
    type Output = FieldElement<P>;

    fn div(self, rhs: Self) -> Self::Output {
        self * &rhs.pow(&(&P::get_prime().to_bigint().unwrap() - &BigInt::from(2u64)))
    }
}

fn rem_euclid(a: &BigInt, b: &BigUint) -> BigUint {
    let sign = a.sign();

    match sign {
        Sign::Minus => {
            let la = a.abs().to_biguint().unwrap();
            let x = la.div_floor(b);
            let x: BigUint = b.mul(&(x + (1u64)));
            x.sub(a.abs().to_biguint().unwrap()).rem(b)
        }
        _ => a.abs().to_biguint().unwrap().rem(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a: FieldElement<Prime29> = FieldElement::new_from_u64(1).unwrap();
        let b = FieldElement::new_from_u64(2).unwrap();
        let c = FieldElement::new_from_u64(28).unwrap();

        assert_eq!(&a + &b, FieldElement::new_from_u64(3).unwrap());
        assert_eq!(&a + &c, FieldElement::new_from_u64(0).unwrap());
    }

    #[test]
    fn add_1_5_1() {
        let a: FieldElement<Prime13> = FieldElement::new_from_u64(7).unwrap();
        let b = FieldElement::new_from_u64(12).unwrap();
        let c = FieldElement::new_from_u64(6).unwrap();

        assert_eq!(&a + &b, c);
    }

    #[test]
    fn mul_1_6_1() {
        let a: FieldElement<Prime13> = FieldElement::new_from_u64(3).unwrap();
        let b = FieldElement::new_from_u64(12).unwrap();
        let c = FieldElement::new_from_u64(10).unwrap();

        assert_eq!(&a * &b, c);
    }

    #[test]
    fn pow_1_6_2() {
        let a: FieldElement<Prime13> = FieldElement::new_from_u64(3).unwrap();
        let b = FieldElement::new_from_u64(1).unwrap();

        assert_eq!(a.pow(&BigInt::from(3u64)), b);
    }

    #[test]
    fn div_test() {
        let a: FieldElement<Prime19> = FieldElement::new_from_u64(2).unwrap();
        let b = FieldElement::new_from_u64(7).unwrap();

        assert_eq!(&a / &b, FieldElement::new_from_u64(3).unwrap());

        let a: FieldElement<Prime19> = FieldElement::new_from_u64(7).unwrap();
        let b = FieldElement::new_from_u64(5).unwrap();

        assert_eq!(&a / &b, FieldElement::new_from_u64(9).unwrap());
    }

    #[test]
    fn pow_minus() {
        let a: FieldElement<Prime13> = FieldElement::new_from_u64(12).unwrap();
        let b = a.pow(
            &(Prime13::get_prime() - BigUint::from(4u64))
                .to_bigint()
                .unwrap(),
        );
        let c = a.pow(&BigInt::from(-3));

        assert_eq!(b, c);
    }

    #[test]
    fn sub_test() {
        let a: FieldElement<Prime29> = FieldElement::new_from_u64(1).unwrap();
        let b = FieldElement::new_from_u64(2).unwrap();
        let c = FieldElement::new_from_u64(28).unwrap();

        assert_eq!(&a - &b, FieldElement::new_from_u64(28).unwrap());
        assert_eq!(&a - &c, FieldElement::new_from_u64(2).unwrap());
    }
}
