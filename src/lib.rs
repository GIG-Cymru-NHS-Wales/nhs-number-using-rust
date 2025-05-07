use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// NHS Number is a unique identifier for patients in the National Health
/// Service of England, Wales, and the Isle of Man.
///
/// Reference: 
/// 
/// * [National Health Service (NHS)](https://en.wikipedia.org/wiki/National_Health_Service)
/// 
/// * [NHS Number](https://en.wikipedia.org/wiki/NHS_number)
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
struct NHSNumber {
    digits: [i8; 10],
}

/// NHS Number Parse Error, which is for the implementation `FromStr`.
#[derive(Debug, PartialEq, Eq)]
struct NHSNumberParseError;

impl NHSNumber {

    /// Create a new NHS Number instance with the provided digits.
    #[allow(dead_code)]
    fn new(digits: [i8; 10]) -> Self {
        NHSNumber { digits }
    }

    // Get the NHS Number check digit i.e. the last digit.
    #[allow(dead_code)]
    fn check_digit(&self) -> i8 {
        self.digits[9]
    }

    // Calculate the NHS Number check digit using a checksum algorithm.
    #[allow(dead_code)]
    fn calculate_check_digit(&self) -> i8 {
        let sum: usize = self.digits
        .iter()
        .take(9)
        .enumerate()
        .map(|(i, &d)| 
            d as usize * (10 - i as usize)
        ).sum();
        ((11 - (sum % 11)) % 10) as i8
    }

    /// Validate the NHS number check digit equals the calculated check digit.
    #[allow(dead_code)]
    fn validate_check_digit(&self) -> bool {
        self.check_digit() == self.calculate_check_digit()
    }

}

/// The NHS Number is a 10-digit number with spaces for formatting:
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

/// Implement the `FromStr` trait for NHSNumber to allow parsing from a string.
/// 
/// This parser allows for optional spcae separators in the NHS number string,
/// so long as the space separators are in their expected positions.
/// 
impl FromStr for NHSNumber {
    type Err = NHSNumberParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 10 || chars.len() > 13 { return Err(NHSNumberParseError); } // Early exit for invalid length
        let mut digits: [i8; 10] = [0; 10];
        let mut i = 0;
        digits[0] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[1] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[2] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        if chars[i] == ' ' { i = i + 1; }
        digits[3] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[4] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[5] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        if chars[i] == ' ' { i = i + 1; }
        digits[6] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[7] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[8] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        digits[9] = chars[i].to_digit(10).ok_or(NHSNumberParseError)? as i8; i = i + 1;
        if chars.len() != i { return Err(NHSNumberParseError); }
        Ok(NHSNumber { digits: digits })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let a: NHSNumber = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 0]);
        let actual: i8 = a.calculate_check_digit();
        let expect: i8 = 9;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_validate_check_digit() {
        {
            let a: NHSNumber = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 9]);
            assert_eq!(a.validate_check_digit(), true);
        }
        {
            let a: NHSNumber = NHSNumber::new([9, 4, 3, 4, 7, 6, 5, 9, 1, 0]);
            assert_eq!(a.validate_check_digit(), false);
        }
    }

    #[test]
    fn test_from_str_with_spaces() {
        let s = String::from("012 345 6789");
        let actual: NHSNumber = NHSNumber::from_str(&s).unwrap();
        let expect: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_str_without_spaces() {
        let s = String::from("0123456789");
        let actual: NHSNumber = NHSNumber::from_str(&s).unwrap();
        let expect: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_str_with_wrong_characters() {
        let s = String::from("012-345-6789");
        let result: Result<NHSNumber, NHSNumberParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_leading_space() {
        let s = String::from(" 012 345 6789");
        let result: Result<NHSNumber, NHSNumberParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_inner_space() {
        let s = String::from("012  345  6789");
        let result: Result<NHSNumber, NHSNumberParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_trailing_space() {
        let s = String::from("012 345 6789 ");
        let result: Result<NHSNumber, NHSNumberParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

}
