use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use wasm_bindgen::prelude::*;

/// 2D Point representation with high-precision 64-bit floats.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Computes the 2D cross determinant of two vectors:
/// det([p1, p2]) = p1.x * p2.y - p1.y * p2.x
pub fn cross_determinant(p1: Point, p2: Point) -> f64 {
    p1.x * p2.y - p1.y * p2.x
}

/// Computes the absolute polygon area using Gauss's shoelace formula.
/// Supports arbitrary non-self-intersecting polygons with vertices listed in order.
pub fn shoelace_area(points: &[Point]) -> f64 {
    signed_shoelace_area(points).abs()
}

/// Computes the signed shoelace area (positive if counter-clockwise, negative if clockwise).
pub fn signed_shoelace_area(points: &[Point]) -> f64 {
    let n = points.len();
    if n < 3 {
        return 0.0;
    }

    let mut sum = 0.0;
    for i in 0..n {
        let next = (i + 1) % n;
        sum += cross_determinant(points[i], points[next]);
    }

    0.5 * sum
}

/// Calculates the nth Fibonacci number F(n).
/// F(0) = 0, F(1) = 1, F(2) = 1, F(3) = 2, F(4) = 3, F(5) = 5, F(6) = 8, F(7) = 13, etc.
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// Results of evaluating Cassini's Identity for a given index n:
/// F_{n-1} * F_{n+1} - F_n^2 = (-1)^n
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CassiniResult {
    pub n: u32,
    pub f_n_minus_1: u64,
    pub f_n: u64,
    pub f_n_plus_1: u64,
    pub product_extremes: u64,
    pub square_middle: u64,
    pub difference: i64,
    pub expected_sign: i64,
    pub identity_holds: bool,
}

/// Evaluates Cassini's Identity at index n.
/// For the Missing Square puzzle, n = 6:
/// F_5 = 5, F_6 = 8, F_7 = 13
/// 5 * 13 - 8^2 = 65 - 64 = 1 = (-1)^6.
/// This exactly explains the 1 unit area discrepancy between pieces and enclosing triangle.
pub fn evaluate_cassini(n: u32) -> CassiniResult {
    assert!(n >= 1, "n must be >= 1 for Cassini's Identity");
    let f_n_minus_1 = fibonacci(n - 1);
    let f_n = fibonacci(n);
    let f_n_plus_1 = fibonacci(n + 1);

    let product_extremes = f_n_minus_1 * f_n_plus_1;
    let square_middle = f_n * f_n;
    let difference = product_extremes as i64 - square_middle as i64;
    let expected_sign = if n % 2 == 0 { 1 } else { -1 };
    let identity_holds = difference == expected_sign;

    CassiniResult {
        n,
        f_n_minus_1,
        f_n,
        f_n_plus_1,
        product_extremes,
        square_middle,
        difference,
        expected_sign,
        identity_holds,
    }
}

/// Calculates the theoretical Y coordinate of the straight hypotenuse
/// spanning (0,0) to (13,5): y = (5.0 / 13.0) * x.
pub fn straightedge_y(x: f64) -> f64 {
    (5.0 / 13.0) * x
}

/// Calculates the vertical deflection between a given point and the straightedge.
/// Positive means the point is above the straightedge (convex bulge).
/// Negative means the point is below the straightedge (concave dip).
pub fn straightedge_deflection(p: Point) -> f64 {
    p.y - straightedge_y(p.x)
}

/// Computes the angle in degrees between two lines with slopes m1 and m2.
/// tan(theta) = |(m2 - m1) / (1 + m1 * m2)|
pub fn angle_between_slopes(m1: f64, m2: f64) -> f64 {
    let delta = (m2 - m1) / (1.0 + m1 * m2);
    delta.abs().atan().to_degrees()
}

/// Area of the lozenge / parallelogram formed by the two broken hypotenuses:
/// Vertices: (0,0), (5,2), (13,5), (8,3).
pub fn mystery_lozenge_points() -> [Point; 4] {
    [
        Point::new(0.0, 0.0),
        Point::new(5.0, 2.0),
        Point::new(13.0, 5.0),
        Point::new(8.0, 3.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn test_cassini_identity_for_curry_puzzle() {
        let res = evaluate_cassini(6);
        assert_eq!(res.f_n_minus_1, 5);
        assert_eq!(res.f_n, 8);
        assert_eq!(res.f_n_plus_1, 13);
        assert_eq!(res.product_extremes, 65);
        assert_eq!(res.square_middle, 64);
        assert_eq!(res.difference, 1);
        assert!(res.identity_holds);
    }

    #[test]
    fn test_mystery_lozenge_area_is_exactly_one() {
        let pts = mystery_lozenge_points();
        let area = shoelace_area(&pts);
        assert!((area - 1.0).abs() < 1e-10, "Lozenge area must be exactly 1.0");
    }

    #[test]
    fn test_slopes_and_angle() {
        let slope_red = 3.0 / 8.0;   // 0.375
        let slope_green = 2.0 / 5.0; // 0.400
        let angle = angle_between_slopes(slope_red, slope_green);
        // arctan( (0.4 - 0.375) / (1 + 0.4 * 0.375) ) = arctan( 0.025 / 1.15 ) = ~1.24536 degrees
        assert!((angle - 1.24536).abs() < 0.001);
    }

    #[test]
    fn test_straightedge_deflection() {
        // Point (8,3) is in Config A: 3 - (5/13)*8 = 3 - 40/13 = -1/13 = -0.076923...
        let def_a = straightedge_deflection(Point::new(8.0, 3.0));
        assert!((def_a - (-1.0 / 13.0)).abs() < 1e-10);

        // Point (5,2) is in Config B: 2 - (5/13)*5 = 2 - 25/13 = +1/13 = +0.076923...
        let def_b = straightedge_deflection(Point::new(5.0, 2.0));
        assert!((def_b - (1.0 / 13.0)).abs() < 1e-10);
    }
}
