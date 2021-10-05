use crate::point::Point;

pub trait EllipticCurve {
    fn on(point: &impl Point) -> bool {
        point.y().powi(2) == point.x().powi(3) + Self::a() * point.x() + Self::b()
    }

    fn a() -> f64;
    fn b() -> f64;
}

#[derive(Debug, PartialEq)]
pub struct Secp256k1;

impl EllipticCurve for Secp256k1 {
    fn a() -> f64 {
        0.0
    }

    fn b() -> f64 {
        7.0
    }
}

#[derive(Debug, PartialEq)]
pub struct TestEllipticCurve;

impl EllipticCurve for TestEllipticCurve {
    fn a() -> f64 {
        5.0
    }

    fn b() -> f64 {
        7.0
    }
}
