use std::sync::LazyLock;
use rand::Rng;
use crate::NHSNumber;

/// Get the NHS Number testable range minimum value.
/// This number is valid but is never going to be issued.
///
#[allow(dead_code)]
pub static TESTABLE_MIN: LazyLock<NHSNumber> = LazyLock::new(|| {
    NHSNumber { digits: [9, 9, 9, 0, 0, 0, 0, 0, 0, 0] }
});

/// Get the NHS Number testable range maximum value.
/// This number is valid but is never going to be issued.
///
#[allow(dead_code)]
pub static TESTABLE_MAX: LazyLock<NHSNumber> = LazyLock::new(|| {
    NHSNumber { digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9] }
});

/// Generate a NHS Number testable range random sample.
/// The generated number is valid but is never going to be issued.
///
#[allow(dead_code)]
pub fn testable_random_sample() -> NHSNumber {
    let mut rng = rand::rng();
    NHSNumber {
        digits: [
            9,
            9,
            9,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
            rng.random_range(0..=9) as i8,
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let a = testable_random_sample();
        assert!(a >= *TESTABLE_MIN);
        assert!(a <= *TESTABLE_MAX);
    }

}
