type Fraction = (i32, i32);

/// Add 2 fractions
pub fn add((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let (n3, d3): Fraction = (n1 * d2 + n2 * d1, d1 * d2);
    simplify(n3, d3)
}

/// Subtract 2 fractions
pub fn sub((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let (n3, d3): Fraction = (n1 * d2 - n2 * d1, d1 * d2);
    simplify(n3, d3)
}

/// Multiply 2 fractions
pub fn mul((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let (n3, d3): Fraction = (n1 * n2, d1 * d2);
    simplify(n3, d3)
}

/// Divide 2 fractions
pub fn divide((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let (n3, d3): Fraction = (n1 * d2, d1 * n2);
    simplify(n3, d3)
}

/// Calculate the Highest common factor between 2 numbers
pub fn hcf(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Create a fraction simplifying with the arguments simplified by the `hcf`
pub fn simplify(n: i32, d: i32) -> Fraction {
    let t = hcf(n, d);
    (n / t, d / t)
}
