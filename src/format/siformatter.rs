use druid::text::format::{Formatter, Validation, ValidationError};
use druid::text::Selection;
use std::error::Error;

pub struct SiFormatter {
    unit_symbol: &'static str,
    decimal_separator: char,
}

impl SiFormatter {
    /// A formatter for millimeters.
    pub const MILLIMETERS: SiFormatter = SiFormatter {
        unit_symbol: "mm",
        decimal_separator: '.',
    };

    /// A formatter for meters.
    pub const METERS: SiFormatter = SiFormatter {
        unit_symbol: "m",
        decimal_separator: '.',
    };

    /// A formatter for meters.
    pub const WATT_PEAK: SiFormatter = SiFormatter {
        unit_symbol: "Wp",
        decimal_separator: '.',
    };
}

/// Errors returned by [`NaiveCurrencyFormatter`].
#[derive(Debug, Clone)]
pub enum DistanceValidationError {
    Parse(std::num::ParseFloatError),
    InvalidChar(char),
    TooManyCharsAfterDecimal,
}

impl std::fmt::Display for DistanceValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DistanceValidationError::InvalidChar(c) => write!(f, "Invalid character '{}'", c),
            DistanceValidationError::Parse(err) => write!(f, "Parse failed: {}", err),
            DistanceValidationError::TooManyCharsAfterDecimal => {
                write!(f, "Too many characters after decimal")
            }
        }
    }
}

impl Error for DistanceValidationError {

}

impl Formatter<f64> for SiFormatter {
    fn format(&self, value: &f64) -> String {
        if !value.is_normal() {
            format!("0{}0{}", self.decimal_separator, self.unit_symbol)
        } else {
            format!("{}{}{}{}", *value as i32, self.decimal_separator, (value % 1.0) as i32, self.unit_symbol)
        }
    }

    fn format_for_editing(&self, value: &f64) -> String {
        if !value.is_normal() {
            format!("0{}0", self.decimal_separator)
        } else {
            format!("{}{}{}", *value as i32, self.decimal_separator, (value % 1.0) as i32)
        }
    }

    fn value(&self, input: &str) -> Result<f64, ValidationError> {
        // we need to convert from our naive localized representation back into
        // rust's float representation
        let decimal_pos = input
            .bytes()
            .rposition(|b| b as char == self.decimal_separator);
        let (major, minor) = input.split_at(decimal_pos.unwrap_or_else(|| input.len()));
        let canonical: String = major
            .chars()
            .chain(Some('.'))
            .chain(minor.chars().skip(1))
            .collect();
        canonical
            .parse()
            .map_err(|err| ValidationError::new(DistanceValidationError::Parse(err)))
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        if input.is_empty() {
            return Validation::success();
        }

        let mut char_iter = input.chars();
        if let Some(c) = char_iter.next() {
            if !(c.is_ascii_digit()) {
                return Validation::failure(DistanceValidationError::InvalidChar(c));
            }
        }
        let mut char_iter =
            char_iter.skip_while(|c| c.is_ascii_digit());
        match char_iter.next() {
            None => return Validation::success(),
            Some(c) if c == self.decimal_separator => (),
            Some(c) => return Validation::failure(DistanceValidationError::InvalidChar(c)),
        };

        // we're after the decimal, allow up to 2 digits
        let (d1, d2, d3) = (char_iter.next(), char_iter.next(), char_iter.next());
        match (d1, d2, d3) {
            (_, _, Some(_)) => {
                Validation::failure(DistanceValidationError::TooManyCharsAfterDecimal)
            }
            (Some(c), None, _) if c.is_ascii_digit() => Validation::success(),
            (None, None, _) => Validation::success(),
            (Some(c1), Some(c2), _) if c1.is_ascii_digit() && c2.is_ascii_digit() => {
                Validation::success()
            }
            (Some(c1), Some(other), _) => {
                let bad_char = if c1.is_ascii_digit() { other } else { c1 };
                Validation::failure(DistanceValidationError::InvalidChar(bad_char))
            }
            _ => unreachable!(),
        }
    }
}
