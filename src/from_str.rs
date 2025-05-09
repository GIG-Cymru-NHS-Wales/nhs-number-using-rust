use std::str::FromStr;
use crate::NHSNumber;
use crate::parse_error::ParseError;

/// Implement the `FromStr` trait for NHSNumber to allow parsing from a string.
///
/// This parser allows for optional spcae separators in the NHS Number string,
/// so long as the space separators are in their expected positions.
///
impl FromStr for NHSNumber {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        match chars.len() {
            10 => {
                let mut digits: [i8; 10] = [0; 10];
                for i in 0..10 {
                    digits[i] = chars[i].to_digit(10).ok_or(ParseError)? as i8
                }
                Ok(NHSNumber { digits: digits })
            },
            12 => {
                if chars[3] != ' ' || chars[7] != ' ' { return Err(ParseError); }
                let mut digits: [i8; 10] = [0; 10];
                for i in 0..3 {
                    digits[i] = chars[i].to_digit(10).ok_or(ParseError)? as i8
                }
                for i in 0..3 {
                    digits[i+3] = chars[i+4].to_digit(10).ok_or(ParseError)? as i8
                }
                for i in 0..4 {
                    digits[i+6] = chars[i+8].to_digit(10).ok_or(ParseError)? as i8
                }
                Ok(NHSNumber { digits: digits })        
            },
            _ => { return Err(ParseError); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_length_10_without_spaces() {
        let s = String::from("0123456789");
        let actual: NHSNumber = NHSNumber::from_str(&s).unwrap();
        let expect: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_str_with_length_12_with_spaces() {
        let s = String::from("012 345 6789");
        let actual: NHSNumber = NHSNumber::from_str(&s).unwrap();
        let expect: NHSNumber = NHSNumber::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_from_str_with_wrong_characters() {
        let s = String::from("012-345-6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_leading_space() {
        let s = String::from(" 012 345 6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_first_space_without_last_space() {
        let s = String::from("012 3456789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_without_first_space_with_last_space() {
        let s = String::from("012345 6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_inner_space() {
        let s = String::from("012  345  6789");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_trailing_space() {
        let s = String::from("012 345 6789 ");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_with_wrong_length() {
        let s = String::from("012");
        let result: Result<NHSNumber, ParseError> = NHSNumber::from_str(&s);
        assert!(result.is_err());
    }

}
