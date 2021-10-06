use crate::field::Field;
use crate::point::Point;
use num::traits::real::Real;
use num::BigInt;
use num_traits::Pow;
use std::ops::{Add, AddAssign, Mul};

pub trait EllipticCurve<T> {
    fn on<'a>(point: &impl Point<T>) -> bool
    where
        T: Field,
    {
        if point.is_finite() {
            T::eq(
                &(T::from(point.y().unwrap().pow(BigInt::from(2)))),
                &(T::from(
                    T::from(
                        T::from(point.x().unwrap().pow(BigInt::from(3)))
                            + T::from(Self::a() * point.x().unwrap()),
                    ) + Self::b(),
                )),
            )
        } else {
            true
        }
    }

    fn a() -> T;
    fn b() -> T;
}

#[derive(Debug, PartialEq)]
pub struct Secp256k1;

impl<'a, T: Field + From<i64>> EllipticCurve<T> for Secp256k1 {
    fn a() -> T {
        T::from(0)
    }

    fn b() -> T {
        T::from(7)
    }
}

#[derive(Debug, PartialEq)]
pub struct TestEllipticCurve;

impl<'a, T: Field + From<i64>> EllipticCurve<T> for TestEllipticCurve {
    fn a() -> T {
        T::from(5)
    }

    fn b() -> T {
        T::from(7)
    }
}
