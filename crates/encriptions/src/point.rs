use crate::curve::EllipticCurve;
use crate::field::Field;
use num::{BigInt, Float};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub trait Point<T> {
    fn x(&self) -> Option<T>;
    fn y(&self) -> Option<T>;
    fn is_finite(&self) -> bool {
        self.x().is_some() && self.y().is_some()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GeneralPoint<T> {
    Finite { x: T, y: T },
    Infinite,
}

impl<'a, T: Field> GeneralPoint<T> {
    pub fn finite(x: T, y: T) -> Self {
        Self::Finite { x, y }
    }
}

impl<'a, T: Field + Clone> Point<T> for GeneralPoint<T> {
    fn x(&self) -> Option<T> {
        match self {
            Self::Finite { x, .. } => Some((*x).clone()),
            Self::Infinite => None,
        }
    }

    fn y(&self) -> Option<T> {
        match self {
            Self::Finite { y, .. } => Some((*y).clone()),
            Self::Infinite => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointOnCurve<T, C: EllipticCurve<T>>(GeneralPoint<T>, PhantomData<fn() -> C>);

impl<'a, T: Field + Clone, C: EllipticCurve<T>> PointOnCurve<T, C> {
    pub fn new(point: GeneralPoint<T>) -> Option<Self> {
        C::on(&point).then(|| Self(point, PhantomData))
    }

    pub fn x(&self) -> Option<T> {
        self.0.x()
    }

    pub fn y(&self) -> Option<T> {
        self.0.y()
    }
}

impl<T: Field + Clone, C: EllipticCurve<T>> Point<T> for PointOnCurve<T, C> {
    fn x(&self) -> Option<T> {
        self.0.x()
    }

    fn y(&self) -> Option<T> {
        self.0.y()
    }
}

impl<T: Field<Output = T> + Clone, C: EllipticCurve<T>> Mul<PointOnCurve<T, C>> for BigInt {
    type Output = PointOnCurve<T, C>;

    fn mul(self, rhs: PointOnCurve<T, C>) -> Self::Output {
        unimplemented!()
    }
}

impl<T: Field<Output = T> + Clone, C: EllipticCurve<T>> Add for PointOnCurve<T, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (GeneralPoint::Infinite, r) => Self(r, PhantomData),
            (l, GeneralPoint::Infinite) => Self(l, PhantomData),
            (GeneralPoint::Finite { x: x1, y: y1 }, GeneralPoint::Finite { x: x2, y: y2 }) => {
                if x1.eq(&x2) {
                    if y1.ne(&y2) {
                        Self::new(GeneralPoint::Infinite).unwrap()
                    } else {
                        let s = (x1.clone().pow(BigInt::from(2)) * T::from(3) + C::a())
                            / (y1.clone() * T::from(2));

                        let x3 = s.clone().pow(BigInt::from(2)) - x1.clone() - x2.clone();
                        Self::new(GeneralPoint::Finite {
                            x: x3.clone(),
                            y: s.mul(x1.clone() - x3) - y1.clone(),
                        })
                        .unwrap()
                    }
                } else {
                    let s = T::from(
                        T::from((y2.clone() - y1.clone())) / T::from((x2.clone() - x1.clone())),
                    );

                    let x3 = s.clone().pow(BigInt::from(2)) - x1.clone() - x2.clone();
                    Self::new(GeneralPoint::Finite {
                        x: x3.clone(),
                        y: s.mul(x1.clone() - x3) - y1.clone(),
                    })
                    .unwrap()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::{Secp256k1, TestEllipticCurve};
    use crate::field::{f64FieldElement, FiniteFieldElement, Prime223};

    #[test]
    fn create_point_on_curve() {
        assert_eq!(
            PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
                f64FieldElement::from(-1.0),
                f64FieldElement::from(-1.0),
            )),
            Some(PointOnCurve::<_, TestEllipticCurve>(
                GeneralPoint::finite(f64FieldElement::from(-1.0), f64FieldElement::from(-1.0)),
                PhantomData
            ))
        );
        assert_eq!(
            PointOnCurve::<_, TestEllipticCurve>::new(GeneralPoint::finite(
                f64FieldElement::from(-1.0),
                f64FieldElement::from(-2.0)
            )),
            None
        );
    }

    #[test]
    fn point_on_curve_add() {
        let p1 = PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
            f64FieldElement::from(2.0),
            f64FieldElement::from(5.0),
        ))
        .unwrap();
        let p2 = PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
            f64FieldElement::from(-1.0),
            f64FieldElement::from(-1.0),
        ))
        .unwrap();

        assert_eq!(
            p1 + p2,
            PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
                f64FieldElement::from(3.0),
                f64FieldElement::from(-7.0),
            ))
            .unwrap()
        );

        let p1 = PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
            f64FieldElement::from(2.0),
            f64FieldElement::from(5.0),
        ))
        .unwrap();
        let p2 = PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
            f64FieldElement::from(2.0),
            f64FieldElement::from(-5.0),
        ))
        .unwrap();

        assert_eq!(
            p1 + p2,
            PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::Infinite)
                .unwrap()
        );

        let p1 = PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
            f64FieldElement::from(-1.0),
            f64FieldElement::from(-1.0),
        ))
        .unwrap();
        let p2 = PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
            f64FieldElement::from(-1.0),
            f64FieldElement::from(-1.0),
        ))
        .unwrap();

        assert_eq!(
            p1 + p2,
            PointOnCurve::<f64FieldElement, TestEllipticCurve>::new(GeneralPoint::finite(
                f64FieldElement::from(18.0),
                f64FieldElement::from(77.0),
            ))
            .unwrap()
        );
    }

    fn secp256k1_point(
        x: i64,
        y: i64,
    ) -> Option<PointOnCurve<FiniteFieldElement<Prime223>, Secp256k1>> {
        PointOnCurve::<FiniteFieldElement<Prime223>, Secp256k1>::new(GeneralPoint::finite(
            FiniteFieldElement::from(x),
            FiniteFieldElement::from(y),
        ))
    }

    #[test]
    fn curve_on_finite_field() {
        assert!(
            PointOnCurve::<FiniteFieldElement<Prime223>, Secp256k1>::new(GeneralPoint::finite(
                FiniteFieldElement::from(192),
                FiniteFieldElement::from(105)
            ),)
            .is_some()
        );
        assert!(
            PointOnCurve::<FiniteFieldElement<Prime223>, Secp256k1>::new(GeneralPoint::finite(
                FiniteFieldElement::from(17),
                FiniteFieldElement::from(56)
            ),)
            .is_some()
        );
        assert!(
            !(PointOnCurve::<FiniteFieldElement<Prime223>, Secp256k1>::new(GeneralPoint::finite(
                FiniteFieldElement::from(200),
                FiniteFieldElement::from(119)
            ),)
            .is_some())
        );
        assert!(
            PointOnCurve::<FiniteFieldElement<Prime223>, Secp256k1>::new(GeneralPoint::finite(
                FiniteFieldElement::from(1),
                FiniteFieldElement::from(193)
            ),)
            .is_some()
        );
        assert!(
            !(PointOnCurve::<FiniteFieldElement<Prime223>, Secp256k1>::new(GeneralPoint::finite(
                FiniteFieldElement::from(42),
                FiniteFieldElement::from(99)
            ),)
            .is_some())
        );
    }

    #[test]
    fn curve_add_finite_field() {
        let p1 = secp256k1_point(170, 142).unwrap();
        let p2 = secp256k1_point(60, 139).unwrap();
        assert_eq!(p1 + p2, secp256k1_point(220, 181).unwrap());

        let p1 = secp256k1_point(47, 71).unwrap();
        let p2 = secp256k1_point(17, 56).unwrap();
        assert_eq!(p1 + p2, secp256k1_point(215, 68).unwrap());

        let p1 = secp256k1_point(143, 98).unwrap();
        let p2 = secp256k1_point(76, 66).unwrap();
        assert_eq!(p1 + p2, secp256k1_point(47, 71).unwrap());
    }
}
