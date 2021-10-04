use num::{BigUint};

trait Prime {
    fn get_prime() -> BigUint;
}

struct Prime29;

impl GetPrime for Prime29 {
    fn get_prime() -> BigUint {
        BigUint::from(29)
    }
}

#[derive(Debug, PartialEq)]
struct FieldElement<P: Prime>(BigUint);


impl<P: Prime> FieldElement<P> {
    pub fn new(value: BigUint) -> Option<Self> {
        if value >= P::get_prime() {
            None
        } else {
            Self(value)
        }
    }
}
