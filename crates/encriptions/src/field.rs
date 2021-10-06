use num::{BigInt, BigUint, Integer, One, Signed, ToPrimitive};
use num_bigint::{Sign, ToBigInt};
use num_traits::Pow;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub trait Prime {
    fn get_prime() -> BigUint;
}

macro_rules! def_prime_struct {
    ($name: ident, $value: literal) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name;

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
def_prime_struct!(Prime223, 223);

pub trait Field:
    Add<Output = <Self as Field>::Output>
    + Sub<Output = <Self as Field>::Output>
    + Mul<Output = <Self as Field>::Output>
    + Div<Output = <Self as Field>::Output>
    + Sized
    + Pow<BigInt, Output = <Self as Field>::Output>
    + PartialEq
    + From<<Self as Field>::Output>
    + From<Self>
    + From<i64>
{
    type Output: Field;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LimitedFieldElement<P: Prime>(BigUint, PhantomData<P>);

impl<P: Prime> LimitedFieldElement<P> {
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
}

impl<P: Prime> Add for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;

    fn add(self, rhs: Self) -> Self::Output {
        LimitedFieldElement((&self.0 + &rhs.0) % P::get_prime(), PhantomData)
    }
}

impl<P: Prime> Sub for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        LimitedFieldElement((&self.0 + (-rhs).0) % P::get_prime(), PhantomData)
    }
}

impl<P: Prime> Mul for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        LimitedFieldElement((&self.0 * &rhs.0) % P::get_prime(), PhantomData)
    }
}

impl<P: Prime> Neg for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;

    fn neg(self) -> Self::Output {
        LimitedFieldElement(
            rem_euclid(&(-(self.0.to_bigint().unwrap())), &P::get_prime()),
            PhantomData,
        )
    }
}

impl<P: Prime> Div for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.pow(P::get_prime().to_bigint().unwrap() - &BigInt::from(2u64))
    }
}

impl<P: Prime> Pow<BigInt> for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;

    fn pow(self, rhs: BigInt) -> Self::Output {
        let exponent = rem_euclid(&rhs, &(P::get_prime() - BigUint::one()));
        LimitedFieldElement(self.0.modpow(&exponent, &P::get_prime()), PhantomData)
    }
}

impl<P: Prime> From<i64> for LimitedFieldElement<P> {
    fn from(v: i64) -> Self {
        Self::new(rem_euclid(&v.to_bigint().unwrap(), &P::get_prime())).unwrap()
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

impl<'a, P: Prime + PartialEq> Field for LimitedFieldElement<P> {
    type Output = LimitedFieldElement<P>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct f64FieldElement(f64);

impl Add for f64FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for f64FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for f64FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for f64FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Pow<BigInt> for f64FieldElement {
    type Output = <Self as Field>::Output;

    fn pow(self, rhs: BigInt) -> Self::Output {
        Self(self.0.powi(rhs.to_i32().unwrap()))
    }
}

impl From<f64> for f64FieldElement {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl From<i64> for f64FieldElement {
    fn from(v: i64) -> Self {
        Self(v.to_f64().unwrap())
    }
}

impl<'a> Field for f64FieldElement {
    type Output = Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a: LimitedFieldElement<Prime29> = LimitedFieldElement::new_from_u64(1).unwrap();
        let b = LimitedFieldElement::new_from_u64(2).unwrap();
        let c = LimitedFieldElement::new_from_u64(28).unwrap();

        assert_eq!(a.clone() + b, LimitedFieldElement::new_from_u64(3).unwrap());
        assert_eq!(a + c, LimitedFieldElement::new_from_u64(0).unwrap());
    }

    #[test]
    fn add_1_5_1() {
        let a: LimitedFieldElement<Prime13> = LimitedFieldElement::new_from_u64(7).unwrap();
        let b = LimitedFieldElement::new_from_u64(12).unwrap();
        let c = LimitedFieldElement::new_from_u64(6).unwrap();

        assert_eq!(a + b, c);
    }

    #[test]
    fn mul_1_6_1() {
        let a: LimitedFieldElement<Prime13> = LimitedFieldElement::new_from_u64(3).unwrap();
        let b = LimitedFieldElement::new_from_u64(12).unwrap();
        let c = LimitedFieldElement::new_from_u64(10).unwrap();

        assert_eq!(a * b, c);
    }

    #[test]
    fn pow_1_6_2() {
        let a: LimitedFieldElement<Prime13> = LimitedFieldElement::new_from_u64(3).unwrap();
        let b = LimitedFieldElement::new_from_u64(1).unwrap();

        assert_eq!(a.pow(BigInt::from(3u64)), b);
    }

    #[test]
    fn div_test() {
        let a: LimitedFieldElement<Prime19> = LimitedFieldElement::new_from_u64(2).unwrap();
        let b = LimitedFieldElement::new_from_u64(7).unwrap();

        assert_eq!(a / b, LimitedFieldElement::new_from_u64(3).unwrap());

        let a: LimitedFieldElement<Prime19> = LimitedFieldElement::new_from_u64(7).unwrap();
        let b = LimitedFieldElement::new_from_u64(5).unwrap();

        assert_eq!(a / b, LimitedFieldElement::new_from_u64(9).unwrap());
    }

    #[test]
    fn pow_minus() {
        let a: LimitedFieldElement<Prime13> = LimitedFieldElement::new_from_u64(12).unwrap();
        let b = a.clone().pow(
            (Prime13::get_prime() - BigUint::from(4u64))
                .to_bigint()
                .unwrap(),
        );
        let c = a.pow(BigInt::from(-3));

        assert_eq!(b, c);
    }

    #[test]
    fn sub_test() {
        let a: LimitedFieldElement<Prime29> = LimitedFieldElement::new_from_u64(1).unwrap();
        let b = LimitedFieldElement::new_from_u64(2).unwrap();
        let c = LimitedFieldElement::new_from_u64(28).unwrap();

        assert_eq!(
            a.clone() - b,
            LimitedFieldElement::new_from_u64(28).unwrap()
        );
        assert_eq!(a - c, LimitedFieldElement::new_from_u64(2).unwrap());
    }
}
