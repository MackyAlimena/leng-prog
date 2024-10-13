use std::ops;

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction(pub i32, pub i32);

impl Fraction {
    pub fn add(&self, other:Fraction) -> Fraction {
        let numerator = self.0 * other.1 + other.0 * self.1;
        let denominator = self.1 * other.1;
        simplify(numerator, denominator)
    }

    pub fn sub(&self, other: Fraction) -> Fraction {
        let numerator = self.0 * other.1 - other.0 * self.1;
        let denominator = self.1 * other.1;
        simplify(numerator, denominator)
    }

    pub fn mul(&self, other: Fraction) -> Fraction {
        let numerator = self.0 * other.0;
        let denominator = self.1 * other.1;
        simplify(numerator, denominator)
    }

    pub fn div(&self, other:Fraction) -> Fraction {
        let numerator = self.0 * other.1;
        let denominator = self.1 * other.0;
        simplify(numerator, denominator)
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        Fraction::add(&self, other)
    }
}

impl ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other:Fraction) -> Fraction {
        Fraction::sub(&self, other)    }
}

impl ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        Fraction::mul(&self, other)    }
}

impl ops::Div for Fraction{
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
        Fraction::div(&self, other)    }
}

/// Calculate the Highest common factor between 2 numbers
fn hcf(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { hcf(b, a % b) }
}

fn simplify(n: i32, d: i32) -> Fraction {
    let h = hcf(n, d);
    Fraction(n/h, d/h)
}