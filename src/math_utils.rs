use std::ops::*;

pub trait Scalar:
    PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Neg<Output = Self>
    + Copy
    + Sized
{
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn zero() -> Self;
    fn one() -> Self;

    fn abs(&self) -> Self;

    fn min(self, other: Self) -> Self {
        if self < other {
            return self;
        }
        other
    }
    fn max(self, other: Self) -> Self {
        if self > other {
            return self;
        }
        other
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            return min;
        }
        if self > max {
            return max;
        }
        self
    }
}

// Returns the number of ways to choose r items from n items
pub fn choose(n: i32, r: i32) -> i32 {
    if r < 0 || r > n {
        return 0;
    }

    let mut result = 1;
    for i in 0..r {
        result *= n - i;
        result /= i + 1;
    }
    result
}

// Returns 1 * 2 * ... * n
pub fn factorial(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

pub fn nth_fibonaci_f64(n: i32) -> f64 {
    let phi = (1f64 + 5f64.sqrt()) / 2f64;
    ((phi.powi(n) - (-phi).powi(-n)) / 5f64.sqrt()).round()
}

// Counts the number of n-simplexes in a d-simplex
pub fn count_n_simplexes(d: i32, n: i32) -> i32 {
    choose(d + 1, n + 1)
}

// Counts the number of n-hypercubes in a d-hypercube
pub fn count_n_hypercubes(d: i32, n: i32) -> i32 {
    if n > d {
        return 0;
    }
    choose(d, n) * (1 << (d - n))
}
