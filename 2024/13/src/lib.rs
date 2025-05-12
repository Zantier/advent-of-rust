// Import the necessary modules
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    // 1. Add variants here (read description)
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

trait StrExt: Sized {
    fn err_if_empty(&self, error: ParseError) -> Result<Self, ParseError>;
}

impl StrExt for &str {
    fn err_if_empty(&self, error: ParseError) -> Result<Self, ParseError> {
        if self.is_empty() {
            Err(error)
        } else {
            Ok(self)
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ParseError::NoName => "Name field is missing",
            ParseError::NoGoodDeeds => "Good deeds field is missing",
            ParseError::NoBadDeeds => "Bad deeds field is missing",
            ParseError::InvalidGoodDeeds => "Good deeds value is invalid",
            ParseError::InvalidBadDeeds => "Bad deeds value is invalid",
        };

        f.write_str(message)?;
        Ok(())
    }
}

// 2. Implement the Error trait for ParseError
impl Error for ParseError {
}

#[derive(Debug, PartialEq)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Kid { name, niceness }
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        // 3. Update the code to return meaningful errors
        let mut fields = csv_row.split(',');
        let name = fields
            .next()
            .ok_or(ParseError::NoName)?
            .err_if_empty(ParseError::NoName)?
            .to_string();
        let good_deeds = fields
            .next()
            .ok_or(ParseError::NoGoodDeeds)?
            .err_if_empty(ParseError::NoGoodDeeds)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidGoodDeeds)?;
        let bad_deeds = fields
            .next()
            .ok_or(ParseError::NoBadDeeds)?
            .err_if_empty(ParseError::NoBadDeeds)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidBadDeeds)?;

        Ok(Kid::new(name, good_deeds, bad_deeds))
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

#[cfg(test)]
mod test {
    use crate::{Kid, ParseError};

    #[test]
    fn test_errors() {
        assert_eq!(Kid::parse_row(""), Err(ParseError::NoName));
        assert_eq!(Kid::parse_row("Alice"), Err(ParseError::NoGoodDeeds));
        assert_eq!(Kid::parse_row("Alice,,1"), Err(ParseError::NoGoodDeeds));
        assert_eq!(Kid::parse_row("Alice,3"), Err(ParseError::NoBadDeeds));
        assert_eq!(Kid::parse_row("Alice,3,"), Err(ParseError::NoBadDeeds));
        assert_eq!(Kid::parse_row("Alice,a,1"), Err(ParseError::InvalidGoodDeeds));
        assert_eq!(Kid::parse_row("Alice,3,a"), Err(ParseError::InvalidBadDeeds));
    }
}
