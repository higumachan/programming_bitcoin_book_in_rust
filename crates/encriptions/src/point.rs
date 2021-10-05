use crate::curve::EllipticCurve;
use num::Float;
use std::marker::PhantomData;

pub trait Point {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GeneralPoint {
    Finite { x: f64, y: f64 },
    Infinite,
}

impl GeneralPoint {
    pub fn finite(x: f64, y: f64) -> Self {
        Self::Finite { x, y }
    }
}

impl Point for GeneralPoint {
    fn x(&self) -> f64 {
        match self {
            Self::Finite { x, .. } => *x,
            Self::Infinite => f64::infinity(),
        }
    }

    fn y(&self) -> f64 {
        match self {
            Self::Finite { y, .. } => *y,
            Self::Infinite => f64::infinity(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointOnCurve<C: EllipticCurve>(GeneralPoint, PhantomData<fn() -> C>);

impl<C: EllipticCurve> PointOnCurve<C> {
    pub fn new(point: GeneralPoint) -> Option<Self> {
        C::on(&point).then(|| Self(point, PhantomData))
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }
}

impl<C: EllipticCurve> Point for PointOnCurve<C> {
    fn x(&self) -> f64 {
        self.0.x()
    }

    fn y(&self) -> f64 {
        self.0.y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::{Secp256k1, TestEllipticCurve};

    #[test]
    fn create_point_on_curve() {
        assert_eq!(
            PointOnCurve::<TestEllipticCurve>::new(GeneralPoint::finite(-1.0, -1.0)),
            Some(PointOnCurve::<TestEllipticCurve>(
                GeneralPoint::finite(-1.0, -1.0),
                PhantomData
            ))
        );
        assert_eq!(
            PointOnCurve::<TestEllipticCurve>::new(GeneralPoint::finite(-1.0, -2.0)),
            None
        );
    }
}
