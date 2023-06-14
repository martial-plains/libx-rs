use alloc::string::{String, ToString};

use crate::num::Number;

#[repr(usize)]
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
    pub fn number(&self, string: &str) -> Result<Number, String> {
        string.parse()
    }

    pub fn string(&self, number: &Number) -> String {
        number.to_string()
    }
}
