use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::HashSet;

pub mod numbers;

#[derive(Debug, Clone)]
pub struct ByteCountFormatter {
    allowed_units: HashSet<ByteCountFormatterUnits>,
    count_style: Option<ByteCountFormatterCountStyle>,
    includes_unit: bool,
    includes_count: bool,
}

impl ByteCountFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts a byte count into a string without using dynamic dispatch.
    pub fn string_from_byte_count(&self, byte_count: i128) -> String {
        let mut allowed_units = Vec::new();

        if self.allowed_units().is_empty()
            || self
                .allowed_units()
                .contains(&ByteCountFormatterUnits::UseAll)
        {
            allowed_units.push(ByteCountFormatterUnits::UseBytes);
            allowed_units.push(ByteCountFormatterUnits::UseKB);
            allowed_units.push(ByteCountFormatterUnits::UseMB);
            allowed_units.push(ByteCountFormatterUnits::UseGB);
            allowed_units.push(ByteCountFormatterUnits::UseTB);
            allowed_units.push(ByteCountFormatterUnits::UsePB);
            allowed_units.push(ByteCountFormatterUnits::UseEB);
            allowed_units.push(ByteCountFormatterUnits::UseZB);
            allowed_units.push(ByteCountFormatterUnits::UseYBOrHigher);
        } else {
            for units in self.allowed_units() {
                allowed_units.push(*units)
            }
        }

        let mut unit_str: &str = "bytes";

        let bytes = match byte_count {
            ..=999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseBytes) {
                    unit_str = if byte_count > 0 { "byte" } else { "bytes" };
                }

                byte_count
            }

            1_000..=999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseKB) {
                    unit_str = "KB";
                }

                byte_count / 10_i128.pow(3)
            }

            1_000_000..=999_999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseMB) {
                    unit_str = "MB";
                }

                byte_count / 10_i128.pow(6)
            }

            1_000_000_000..=999_999_999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseGB) {
                    unit_str = "GB";
                }

                byte_count / 10_i128.pow(9)
            }

            1_000_000_000_000..=999_999_999_999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseTB) {
                    unit_str = "TB";
                }

                byte_count / 10_i128.pow(12)
            }

            1_000_000_000_000_000..=999_999_999_999_999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UsePB) {
                    unit_str = "PB";
                }

                byte_count / 10_i128.pow(15)
            }

            1_000_000_000_000_000_000..=999_999_999_999_999_999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseEB) {
                    unit_str = "EB";
                }

                byte_count / 10_i128.pow(18)
            }

            1_000_000_000_000_000_000_000..=999_999_999_999_999_999_999_999 => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseZB) {
                    unit_str = "ZB";
                }

                byte_count / 10_i128.pow(21)
            }

            _ => {
                if allowed_units.contains(&ByteCountFormatterUnits::UseYBOrHigher) {
                    unit_str = "YB";
                }

                byte_count / 10_i128.pow(24)
            }
        };

        format!(
            "{count}{space}{unit}",
            count = if self.includes_count {
                bytes.to_string()
            } else {
                String::new()
            },
            space = if self.includes_count && self.includes_unit {
                " "
            } else {
                ""
            },
            unit = if self.includes_unit { unit_str } else { "" }
        )
    }

    pub fn allowed_units(&self) -> &HashSet<ByteCountFormatterUnits> {
        &self.allowed_units
    }

    pub fn set_allowed_units(&mut self, allowed_units: HashSet<ByteCountFormatterUnits>) {
        self.allowed_units = allowed_units;
    }

    pub fn count_style(&self) -> Option<ByteCountFormatterCountStyle> {
        self.count_style
    }

    pub fn set_count_style(&mut self, count_style: Option<ByteCountFormatterCountStyle>) {
        self.count_style = count_style;
    }

    pub fn includes_unit(&self) -> bool {
        self.includes_unit
    }

    pub fn set_includes_unit(&mut self, includes_unit: bool) {
        self.includes_unit = includes_unit;
    }

    pub fn includes_count(&self) -> bool {
        self.includes_count
    }

    pub fn set_includes_count(&mut self, includes_count: bool) {
        self.includes_count = includes_count;
    }
}

impl Default for ByteCountFormatter {
    fn default() -> Self {
        ByteCountFormatter {
            allowed_units: HashSet::new(),
            count_style: None,
            includes_unit: true,
            includes_count: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ByteCountFormatterCountStyle {
    File,
    Memory,
    Decimal,
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ByteCountFormatterUnits {
    UseBytes,
    UseKB,
    UseMB,
    UseGB,
    UseTB,
    UsePB,
    UseEB,
    UseZB,
    UseYBOrHigher,
    UseAll,
}
