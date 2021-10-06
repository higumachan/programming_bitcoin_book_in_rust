use crate::curve::EllipticCurve;
use crate::field::Field;
use num::Float;
use std::marker::PhantomData;

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

impl<'a, T: Field + Clone, C: EllipticCurve<T>> Point<T> for PointOnCurve<T, C> {
    fn x(&self) -> Option<T> {
        self.0.x()
    }

    fn y(&self) -> Option<T> {
        self.0.y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::{Secp256k1, TestEllipticCurve};
    use crate::field::f64FieldElement;

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
}
