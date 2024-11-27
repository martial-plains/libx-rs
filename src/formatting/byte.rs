use alloc::{
    fmt, format,
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::HashMap;

use crate::num::traits::FloatingPoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CountFormatterUnits {
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

impl fmt::Display for CountFormatterUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UseBytes => write!(f, "bytes"),
            Self::UseKB => write!(f, "KB"),
            Self::UseMB => write!(f, "MB"),
            Self::UseGB => write!(f, "GB"),
            Self::UseTB => write!(f, "TB"),
            Self::UsePB => write!(f, "PB"),
            Self::UseEB => write!(f, "EB"),
            Self::UseZB => write!(f, "ZB"),
            Self::UseYBOrHigher => write!(f, "YB"),
            Self::UseAll => write!(f, "All"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CountFormatter {
    pub allowed_units: Vec<CountFormatterUnits>,
    pub includes_unit: bool,
    pub includes_count: bool,
    pub includes_actual_byte_count: bool,
}

impl CountFormatter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts a byte count into a string without using dynamic dispatch.
    #[must_use]
    pub fn string_from_byte_count(&self, byte_count: i128) -> String {
        let mut allowed_units = Vec::new();

        self.get_allowed_units(&mut allowed_units);

        let mut unit_str = String::from("bytes");
        let mut bytes = byte_count;

        if self.allowed_units.contains(&CountFormatterUnits::UseBytes) {
            unit_str = if byte_count == 1 {
                String::from("byte")
            } else {
                String::from("bytes")
            };
        } else if self.allowed_units.contains(&CountFormatterUnits::UseKB) {
            unit_str = "KB".to_string();
            bytes /= 10_i128.pow(3);
        } else if self.allowed_units.contains(&CountFormatterUnits::UseMB) {
            unit_str = "MB".to_string();
            bytes /= 10_i128.pow(6);
        } else if self.allowed_units.contains(&CountFormatterUnits::UseGB) {
            unit_str = "GB".to_string();
            bytes /= 10_i128.pow(9);
        } else if self.allowed_units.contains(&CountFormatterUnits::UseTB) {
            unit_str = "TB".to_string();
            bytes /= 10_i128.pow(12);
        } else if self.allowed_units.contains(&CountFormatterUnits::UsePB) {
            unit_str = "PB".to_string();
            bytes /= 10_i128.pow(15);
        } else if self.allowed_units.contains(&CountFormatterUnits::UseEB) {
            unit_str = "EB".to_string();
            bytes /= 10_i128.pow(18);
        } else if self.allowed_units.contains(&CountFormatterUnits::UseZB) {
            unit_str = "ZB".to_string();
            bytes /= 10_i128.pow(21);
        } else if self
            .allowed_units
            .contains(&CountFormatterUnits::UseYBOrHigher)
        {
            unit_str = "YB".to_string();
            bytes /= 10_i128.pow(24);
        } else {
            let mut units_in_bytes = HashMap::new();
            units_in_bytes.insert(CountFormatterUnits::UseBytes, 0_i128);
            units_in_bytes.insert(CountFormatterUnits::UseKB, 10_i128.pow(3));
            units_in_bytes.insert(CountFormatterUnits::UseMB, 10_i128.pow(6));
            units_in_bytes.insert(CountFormatterUnits::UseGB, 10_i128.pow(9));
            units_in_bytes.insert(CountFormatterUnits::UseTB, 10_i128.pow(12));
            units_in_bytes.insert(CountFormatterUnits::UsePB, 10_i128.pow(15));
            units_in_bytes.insert(CountFormatterUnits::UseEB, 10_i128.pow(18));
            units_in_bytes.insert(CountFormatterUnits::UseZB, 10_i128.pow(21));
            units_in_bytes.insert(CountFormatterUnits::UseYBOrHigher, 10_i128.pow(24));

            let mut closest_value = i128::MAX;

            for unit in allowed_units {
                if units_in_bytes.contains_key(&unit) {
                    let value = units_in_bytes[&unit];
                    if (byte_count - value).abs() < (byte_count - closest_value).abs() {
                        closest_value = value;
                        unit_str = unit.to_string();
                    }
                }
            }

            if closest_value != i128::MAX && bytes != 0 {
                bytes /= closest_value;
            }
        }

        format!(
            "{count}{space}{unit}{actual_count}",
            count = self.format_count(bytes, byte_count),
            space = if self.includes_count && self.includes_unit {
                " "
            } else {
                ""
            },
            unit = if self.includes_unit {
                unit_str
            } else {
                String::new()
            },
            actual_count = if self.includes_actual_byte_count {
                format!(" ({byte_count} bytes)")
            } else {
                String::new()
            }
        )
    }

    #[allow(clippy::cast_possible_truncation)]
    fn format_count(&self, bytes: i128, byte_count: i128) -> String {
        if self.includes_count {
            let whole_number_str = bytes.to_string();
            let decimal_numbers_str = {
                let byte_count_str = byte_count.to_string();
                let mut decimal_part = byte_count_str[byte_count_str
                    .find(&whole_number_str)
                    .expect("Could find whole number within `byte_count`")
                    + whole_number_str.len()..]
                    .to_string();
                if decimal_part.is_empty() {
                    decimal_part = String::from("0.0");
                } else {
                    decimal_part.insert(1, '.');
                }

                let float = decimal_part
                    .parse::<f64>()
                    .expect("Could not parse decimal part to float")
                    .rounded();

                (float as i128).to_string()
            };

            if decimal_numbers_str.chars().all(|c| c == '0') {
                whole_number_str
            } else {
                format!("{whole_number_str}.{decimal_numbers_str}")
            }
        } else {
            String::new()
        }
    }

    fn get_allowed_units(&self, allowed_units: &mut Vec<CountFormatterUnits>) {
        if self.allowed_units.is_empty()
            || self.allowed_units.contains(&CountFormatterUnits::UseAll)
        {
            allowed_units.push(CountFormatterUnits::UseBytes);
            allowed_units.push(CountFormatterUnits::UseKB);
            allowed_units.push(CountFormatterUnits::UseMB);
            allowed_units.push(CountFormatterUnits::UseGB);
            allowed_units.push(CountFormatterUnits::UseTB);
            allowed_units.push(CountFormatterUnits::UsePB);
            allowed_units.push(CountFormatterUnits::UseEB);
            allowed_units.push(CountFormatterUnits::UseZB);
            allowed_units.push(CountFormatterUnits::UseYBOrHigher);
        } else {
            for units in &self.allowed_units {
                allowed_units.push(*units);
            }
        }
    }
}

impl Default for CountFormatter {
    fn default() -> Self {
        Self {
            allowed_units: Vec::new(),
            includes_unit: true,
            includes_count: true,
            includes_actual_byte_count: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_string_from_byte_count() {
        let formatter = CountFormatter::new();

        // Test with default settings
        assert_eq!(formatter.string_from_byte_count(0), "0 bytes");
        assert_eq!(formatter.string_from_byte_count(1023), "1 KB");
        assert_eq!(formatter.string_from_byte_count(1024), "1 KB");
        assert_eq!(formatter.string_from_byte_count(1_048_576), "1 MB");
        assert_eq!(formatter.string_from_byte_count(1_073_741_824), "1.1 GB");
        assert_eq!(
            formatter.string_from_byte_count(1_099_511_627_776),
            "1.1 TB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1_125_899_906_842_624),
            "1.1 PB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1_152_921_504_606_846_976),
            "1.2 EB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1_180_591_620_717_411_303_424),
            "1.2 ZB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1_208_925_819_614_629_174_706_176),
            "1.2 YB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1_237_940_039_285_380_274_899_124_224),
            "1237.9 YB"
        );

        // Test with custom settings
        let mut custom_formatter = CountFormatter::new();
        custom_formatter.allowed_units = vec![
            CountFormatterUnits::UseBytes,
            CountFormatterUnits::UseMB,
            CountFormatterUnits::UseGB,
            CountFormatterUnits::UseYBOrHigher,
        ]
        .into_iter()
        .collect();
        custom_formatter.includes_count = false;
        custom_formatter.includes_unit = false;

        assert_eq!(custom_formatter.string_from_byte_count(0), "");
        assert_eq!(custom_formatter.string_from_byte_count(1023), "");
        assert_eq!(custom_formatter.string_from_byte_count(1_048_576), "");
        assert_eq!(custom_formatter.string_from_byte_count(1_073_741_824), "");
        assert_eq!(
            custom_formatter.string_from_byte_count(1_099_511_627_776),
            ""
        );
        assert_eq!(
            custom_formatter.string_from_byte_count(1_237_940_039_285_380_274_899_124_224),
            ""
        );
    }

    #[test]
    fn test_allowed_units() {
        let mut formatter = CountFormatter::new();

        // Test with default settings
        assert_eq!(formatter.allowed_units, vec![]);

        // Test after setting custom units
        formatter.allowed_units = vec![
            CountFormatterUnits::UseKB,
            CountFormatterUnits::UseMB,
            CountFormatterUnits::UseGB,
        ]
        .into_iter()
        .collect();

        assert_eq!(
            formatter.allowed_units,
            vec![
                CountFormatterUnits::UseKB,
                CountFormatterUnits::UseMB,
                CountFormatterUnits::UseGB,
            ]
        );
    }

    #[test]
    fn test_includes_unit() {
        let mut formatter = CountFormatter::new();

        // Test with default settings
        assert!(formatter.includes_unit);

        // Test after setting custom includes unit value
        formatter.includes_unit = false;
        assert!(!formatter.includes_unit);
    }

    #[test]
    fn test_includes_count() {
        let mut formatter = CountFormatter::new();

        // Test with default settings
        assert!(formatter.includes_count);

        // Test after setting custom includes count value
        formatter.includes_count = false;
        assert!(!formatter.includes_count);
    }
}
