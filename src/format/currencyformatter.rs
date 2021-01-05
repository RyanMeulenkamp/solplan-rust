// Shameless copy of the formatter example https://github.com/linebender/druid/pull/1377

use druid::text::format::{Validation, ValidationError, Formatter};
use druid::text::Selection;

pub struct CurrencyFormatter {
    currency_symbol: char,
    thousands_separator: char,
    decimal_separator: char,
}

#[derive(Debug, Clone)]
pub enum CurrencyValidationError {
    Parse(std::num::ParseFloatError),
    InvalidChar(char),
    TooManyCharsAfterDecimal,
}

impl std::fmt::Display for CurrencyValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CurrencyValidationError::InvalidChar(c) => write!(f, "Invalid character '{}'", c),
            CurrencyValidationError::Parse(err) => write!(f, "Parse failed: {}", err),
            CurrencyValidationError::TooManyCharsAfterDecimal => {
                write!(f, "Too many characters after decimal")
            }
        }
    }
}

impl std::error::Error for CurrencyValidationError {

}

impl CurrencyFormatter {
    /// A formatter for USD.
    pub const DOLLARS: CurrencyFormatter = CurrencyFormatter {
        currency_symbol: '$',
        thousands_separator: ',',
        decimal_separator: '.',
    };

    /// A formatter for euros.
    pub const EUROS: CurrencyFormatter = CurrencyFormatter {
        currency_symbol: '€',
        thousands_separator: '.',
        decimal_separator: ',',
    };

    /// A formatter for british pounds.
    pub const GBP: CurrencyFormatter = CurrencyFormatter {
        currency_symbol: '£',
        thousands_separator: '.',
        decimal_separator: ',',
    };
}

impl Formatter<f64> for CurrencyFormatter {
    fn format(&self, value: &f64) -> String {
        if !value.is_normal() {
            return format!("{}0{}00", self.currency_symbol, self.decimal_separator);
        }

        let mut components = Vec::new();
        let mut major_part = value.abs().trunc() as usize;
        let minor_part = (value.abs().fract() * 100.0).round() as usize;

        let bonus_rounding_dollar = minor_part / 100;

        components.push(format!("{}{:02}", self.decimal_separator, minor_part % 100));
        if major_part == 0 {
            components.push('0'.to_string());
        }

        while major_part > 0 {
            let remain = major_part % 1000;
            major_part /= 1000;
            if major_part > 0 {
                components.push(format!("{}{:03}", self.thousands_separator, remain));
            } else {
                components.push((remain + bonus_rounding_dollar).to_string());
            }
        }
        if value.is_sign_negative() {
            components.push(format!("-{}", self.currency_symbol));
        } else {
            components.push(self.currency_symbol.to_string());
        }

        components.iter().rev().flat_map(|s| s.chars()).collect()
    }

    fn format_for_editing(&self, value: &f64) -> String {
        self.format(value)
            .chars()
            .filter(|c| *c != self.currency_symbol)
            .collect()
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
            .filter(|c| *c != self.thousands_separator)
            .chain(Some('.'))
            .chain(minor.chars().skip(1))
            .collect();
        canonical
            .parse()
            .map_err(|err| ValidationError::new(CurrencyValidationError::Parse(err)))
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        if input.is_empty() {
            return Validation::success();
        }

        let mut char_iter = input.chars();
        if let Some(c) = char_iter.next() {
            if !(c.is_ascii_digit() || c == '-') {
                return Validation::failure(CurrencyValidationError::InvalidChar(c));
            }
        }
        let mut char_iter =
            char_iter.skip_while(|c| c.is_ascii_digit() || *c == self.thousands_separator);
        match char_iter.next() {
            None => return Validation::success(),
            Some(c) if c == self.decimal_separator => (),
            Some(c) => return Validation::failure(CurrencyValidationError::InvalidChar(c)),
        };

        // we're after the decimal, allow up to 2 digits
        let (d1, d2, d3) = (char_iter.next(), char_iter.next(), char_iter.next());
        match (d1, d2, d3) {
            (_, _, Some(_)) => {
                Validation::failure(CurrencyValidationError::TooManyCharsAfterDecimal)
            }
            (Some(c), None, _) if c.is_ascii_digit() => Validation::success(),
            (None, None, _) => Validation::success(),
            (Some(c1), Some(c2), _) if c1.is_ascii_digit() && c2.is_ascii_digit() => {
                Validation::success()
            }
            (Some(c1), Some(other), _) => {
                let bad_char = if c1.is_ascii_digit() { other } else { c1 };
                Validation::failure(CurrencyValidationError::InvalidChar(bad_char))
            }
            _ => unreachable!(),
        }
    }
}
