//! # NHS Number
//!
//! A National Health Service (NHS) Number is a unique number allocated in a shared
//! numbering scheme to registered users of the three public health services in
//! England, Wales, and the Isle of Man.
//!
//! The NHS Number is the key to the identification of patients, especially in
//! delivering safe care across provider organisations, and is required in all new
//! software deployed within the National Health Service (NHS) organizations.
//!
//! References:
//!
//! * [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
//!
//! * [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
//!
//! ## Syntax
//!
//! The current system uses a ten-digit number in '3 3 4' format with the final
//! digit being an error-detecting checksum. Examples given include 987 654 4321.
//!
//! ## Ranges
//!
//! Currently issued numbers are in these ranges:
//!
//! * 300 000 000 to 399 999 999 (England)
//!
//! * 400 000 000 to 499 999 999 (England, Wales, Isle of Man)
//!
//! * 600 000 000 to 799 999 999 (England, Wales, Isle of Man)
//!
//! Unavailable number ranges include:
//!
//! * 320 000 001 to 399 999 999 (allocated to the Northern Irish system)
//!
//! * 010 100 0000 to 311 299 9999 (used for CHI numbers in Scotland)
//!
//! For test purposes, this range is valid but is guaranteed to never be issued:
//!
//! * 999 000 0000 to 999 999 9999
//!
//! ## Checksum
//!
//! The checksum is calculated by multiplying each of the first nine digits by 11
//! minus its position. Using the number 943 476 5919 as an example:
//!
//! * The first digit is 9. This is multiplied by 10.
//!
//! * The second digit is 4. This is multiplied by 9.
//!
//! * And so on until the ninth digit (1) is multiplied by 2.
//!
//! * The result of this calculation is summed. In this example: (9×10) + (4×9) +
//!   (3×8) + (4×7) + (7×6) + (6×5) + (5×4) + (9×3) + (1×2) = 299.
//!
//! * The remainder when dividing this number by 11 is calculated, yielding a number
//!   in the range 0–10, which would be 2 in this case.
//!
//! * Finally, this number is subtracted from 11 to give the checksum in the range
//!   1–11, in this case 9, which becomes the last digit of the NHS Number.
//!
//! * A checksum of 11 is represented by 0 in the final NHS Number. If the checksum
//!   is 10 then the number is not valid.
//!
//! ## Examples
//!
//! ```rust
//! use nhs_number::*;
//! use std::str::FromStr;
//!
//! // Create a new NHS Number with the provided digits.
//! let nhs_number = NHSNumber { digits: [9, 9, 9, 1, 2, 3, 4, 5, 6, 0] };
//!
//! // Create a new NHS Number by converting from a string.
//! let nhs_number = NHSNumber::from_str("999 123 4560").unwrap();
//!
//! // Create a new NHS Number by parsing a string.
//! let nhs_number: NHSNumber = "999 123 4560".parse().unwrap();
//!
//! // Validate a NHS Number using the NHS check digit algorithm.
//! let valid: bool = nhs_number.validate_check_digit();
//!
//! // Create a new NHS Number random sample in the testable range.
//! let sample = NHSNumber::testable_random_sample();
//! ```
//!

use std::fmt;
use serde::{Serialize, Deserialize};

pub mod from_str;
pub mod parse_error;
pub mod testable;
pub use testable::*;

/// NHS Number is a unique identifier for patients in the National Health
/// Service of England, Wales, and the Isle of Man.
///
/// Reference:
///
/// * [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
///
/// * [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct NHSNumber {
    pub digits: [i8; 10],
}

impl NHSNumber {

    /// Create a new NHS Number instance with the provided digits.
    #[allow(dead_code)]
    pub fn new(digits: [i8; 10]) -> Self {
        NHSNumber { digits }
    }

    // Get the NHS Number check digit i.e. the last digit.
    #[allow(dead_code)]
    pub fn check_digit(&self) -> i8 {
        crate::check_digit(self.digits)
    }

    // Calculate the NHS Number check digit using a checksum algorithm.
    #[allow(dead_code)]
    pub fn calculate_check_digit(&self) -> i8 {
        crate::calculate_check_digit(self.digits)
    }

    /// Validate the NHS Number check digit equals the calculated check digit.
    #[allow(dead_code)]
    pub fn validate_check_digit(&self) -> bool {
        crate::check_digit(self.digits) == crate::calculate_check_digit(self.digits)
    }

    /// Validate the NHS Number check digit equals the calculated check digit.
    #[allow(dead_code)]
    pub fn testable_random_sample() -> NHSNumber {
        crate::testable_random_sample()
    }


}

/// Format the NHS Number is a 10-digit number with spaces:
///
/// * 3 digits
/// * space
/// * 3 digits
/// * space
/// * 4 digits
///
impl fmt::Display for NHSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{} {}{}{} {}{}{}{}",
            self.digits[0],
            self.digits[1],
            self.digits[2],
            self.digits[3],
            self.digits[4],
            self.digits[5],
            self.digits[6],
            self.digits[7],
            self.digits[8],
            self.digits[9],
        )
    }
}

//// Functional utilities

/// Format the NHS Number is a 10-digit number with spaces:
///
/// * 3 digits
/// * space
/// * 3 digits
/// * space
/// * 4 digits
///
#[allow(dead_code)]
pub fn format(digits: [i8; 10]) -> String {
    format!(
        "{}{}{} {}{}{} {}{}{}{}",
        digits[0],
        digits[1],
        digits[2],
        digits[3],
        digits[4],
        digits[5],
        digits[6],
        digits[7],
        digits[8],
        digits[9],
    )
}

/// Get the NHS Number check digit i.e. the last digit.
#[allow(dead_code)]
pub fn check_digit(digits: [i8; 10]) -> i8 {
    digits[9]
}

/// Calculate the NHS Number check digit using a checksum algorithm.
#[allow(dead_code)]
fn calculate_check_digit(digits: [i8; 10]) -> i8 {
    let sum: usize = digits
    .iter()
    .take(9)
    .enumerate()
    .map(|(i, &d)|
        d as usize * (10 - i as usize)
    ).sum();
    ((11 - (sum % 11)) % 10) as i8
}

#[cfg(test)]
mod tests {

    mod structure {
        use super::super::*;

        #[test]
        fn test_new() {
            let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual = a.to_string();
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_display() {
            let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual = a.to_string();
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_partial_eq() {
            {
                let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
                let b: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
                assert_eq!(a, b);
            }
            {
                let a: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
                let b: NHSNumber = NHSNumber::new([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
                assert_ne!(a, b);
            }
        }

        #[test]
        fn test_check_digit() {
            let a = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let actual: i8 = a.check_digit();
            let expect: i8 = 9;
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_calculate_check_digit() {
            let a: NHSNumber = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
            let actual: i8 = a.calculate_check_digit();
            let expect: i8 = 0;
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_validate_check_digit() {
            {
                let a: NHSNumber = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 0]);
                assert_eq!(a.validate_check_digit(), true);
            }
            {
                let a: NHSNumber = NHSNumber::new([9, 9, 9, 1, 2, 3, 4, 5, 6, 1]);
                assert_eq!(a.validate_check_digit(), false);
            }
        }

        #[test]
        fn test_testable_random_sample() {
            let a: NHSNumber = NHSNumber::testable_random_sample();
            assert!(a >= *crate::testable::TESTABLE_MIN);
            assert!(a <= *crate::testable::TESTABLE_MAX);
        }

    }

    mod utilities {

        #[test]
        fn test_format() {
            let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let actual = crate::format(digits);
            let expect = "012 345 6789";
            assert_eq!(actual, expect);
        }
   
        #[test]
        fn test_check_digit() {
            let digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let actual: i8 = crate::check_digit(digits);
            let expect: i8 = 9;
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_calculate_check_digit() {
            let digits = [9, 9, 9, 1, 2, 3, 4, 5, 6, 0];
            let actual: i8 = crate::calculate_check_digit(digits);
            let expect: i8 = 0;
            assert_eq!(actual, expect);
        }

    }

}
