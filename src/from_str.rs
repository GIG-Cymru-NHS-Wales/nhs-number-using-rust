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
        if chars.len() < 10 || chars.len() > 13 { return Err(ParseError); } // Early exit for invalid length
        let mut digits: [i8; 10] = [0; 10];
        let mut i = 0;
        digits[0] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[1] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[2] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        if chars[i] == ' ' { i += 1; }
        digits[3] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[4] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[5] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        if chars[i] == ' ' { i += 1; }
        digits[6] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[7] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[8] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        digits[9] = chars[i].to_digit(10).ok_or(ParseError)? as i8; i += 1;
        if chars.len() != i { return Err(ParseError); }
        Ok(NHSNumber { digits: digits })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
