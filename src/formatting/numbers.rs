use alloc::string::{String, ToString};

use crate::num::Number;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum NumberFormatterStyle {
    #[default]
    None,
    Decimal,
    Percent,
    Scientific,
    SpellOut,
    Ordinal,
    Currency,
    CurrencyAccounting,
    CurrencyISOCode,
    CurrencyPlural,
}

unsafe impl Send for NumberFormatterStyle {}
unsafe impl Sync for NumberFormatterStyle {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct NumberFormatter {
    pub number_style: NumberFormatterStyle,
    pub generates_decimal_numbers: bool,
}

impl NumberFormatter {
    /// # Errors
    /// Will return [`Err`] if it's not possible to parse this string slice into the desired type.
    pub fn number(&self, string: &str) -> Result<Number, String> {
        string.parse()
    }

    #[must_use]
    pub fn string(&self, number: &Number) -> String {
        number.to_string()
    }
}
