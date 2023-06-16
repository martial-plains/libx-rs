use alloc::{
    fmt, format,
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::{HashMap, HashSet};

pub mod numbers;

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

impl fmt::Display for ByteCountFormatterUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteCountFormatterUnits::UseBytes => write!(f, "bytes"),
            ByteCountFormatterUnits::UseKB => write!(f, "KB"),
            ByteCountFormatterUnits::UseMB => write!(f, "MB"),
            ByteCountFormatterUnits::UseGB => write!(f, "GB"),
            ByteCountFormatterUnits::UseTB => write!(f, "TB"),
            ByteCountFormatterUnits::UsePB => write!(f, "PB"),
            ByteCountFormatterUnits::UseEB => write!(f, "EB"),
            ByteCountFormatterUnits::UseZB => write!(f, "ZB"),
            ByteCountFormatterUnits::UseYBOrHigher => write!(f, "YB"),
            ByteCountFormatterUnits::UseAll => write!(f, "All"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ByteCountFormatter {
    pub allowed_units: HashSet<ByteCountFormatterUnits>,
    pub includes_unit: bool,
    pub includes_count: bool,
}

impl ByteCountFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts a byte count into a string without using dynamic dispatch.
    pub fn string_from_byte_count(&self, byte_count: i128) -> String {
        let mut allowed_units = Vec::new();

        if self.allowed_units.is_empty()
            || self
                .allowed_units
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
            for units in &self.allowed_units {
                allowed_units.push(*units)
            }
        }

        let mut unit_str = String::from("bytes");
        let mut bytes = byte_count;

        if self
            .allowed_units
            .contains(&ByteCountFormatterUnits::UseBytes)
        {
            unit_str = if byte_count != 1 {
                String::from("bytes")
            } else {
                String::from("byte")
            };
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UseKB) {
            unit_str = "KB".to_string();
            bytes /= 10_i128.pow(3);
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UseMB) {
            unit_str = "MB".to_string();
            bytes /= 10_i128.pow(6);
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UseGB) {
            unit_str = "GB".to_string();
            bytes /= 10_i128.pow(9);
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UseTB) {
            unit_str = "TB".to_string();
            bytes /= 10_i128.pow(12);
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UsePB) {
            unit_str = "PB".to_string();
            bytes /= 10_i128.pow(15);
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UseEB) {
            unit_str = "EB".to_string();
            bytes /= 10_i128.pow(18);
        } else if self.allowed_units.contains(&ByteCountFormatterUnits::UseZB) {
            unit_str = "ZB".to_string();
            bytes /= 10_i128.pow(21);
        } else if self
            .allowed_units
            .contains(&ByteCountFormatterUnits::UseYBOrHigher)
        {
            unit_str = "YB".to_string();
            bytes /= 10_i128.pow(24);
        } else {
            let mut units_in_bytes = HashMap::new();
            units_in_bytes.insert(ByteCountFormatterUnits::UseBytes, 0_i128);
            units_in_bytes.insert(ByteCountFormatterUnits::UseKB, 10_i128.pow(3));
            units_in_bytes.insert(ByteCountFormatterUnits::UseMB, 10_i128.pow(6));
            units_in_bytes.insert(ByteCountFormatterUnits::UseGB, 10_i128.pow(9));
            units_in_bytes.insert(ByteCountFormatterUnits::UseTB, 10_i128.pow(12));
            units_in_bytes.insert(ByteCountFormatterUnits::UsePB, 10_i128.pow(15));
            units_in_bytes.insert(ByteCountFormatterUnits::UseEB, 10_i128.pow(18));
            units_in_bytes.insert(ByteCountFormatterUnits::UseZB, 10_i128.pow(21));
            units_in_bytes.insert(ByteCountFormatterUnits::UseYBOrHigher, 10_i128.pow(24));

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
            unit = if self.includes_unit {
                unit_str
            } else {
                String::new()
            }
        )
    }
}

impl Default for ByteCountFormatter {
    fn default() -> Self {
        ByteCountFormatter {
            allowed_units: HashSet::new(),
            includes_unit: true,
            includes_count: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_string_from_byte_count() {
        let formatter = ByteCountFormatter::new();

        // Test with default settings
        assert_eq!(formatter.string_from_byte_count(0), "0 bytes");
        assert_eq!(formatter.string_from_byte_count(1023), "1 KB");
        assert_eq!(formatter.string_from_byte_count(1024), "1 KB");
        assert_eq!(formatter.string_from_byte_count(1048576), "1 MB");
        assert_eq!(formatter.string_from_byte_count(1073741824), "1 GB");
        assert_eq!(formatter.string_from_byte_count(1099511627776), "1 TB");
        assert_eq!(formatter.string_from_byte_count(1125899906842624), "1 PB");
        assert_eq!(
            formatter.string_from_byte_count(1152921504606846976),
            "1 EB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1180591620717411303424),
            "1 ZB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1208925819614629174706176),
            "1 YB"
        );
        assert_eq!(
            formatter.string_from_byte_count(1237940039285380274899124224),
            "1237 YB"
        );

        // Test with custom settings
        let mut custom_formatter = ByteCountFormatter::new();
        custom_formatter.allowed_units = vec![
            ByteCountFormatterUnits::UseBytes,
            ByteCountFormatterUnits::UseMB,
            ByteCountFormatterUnits::UseGB,
            ByteCountFormatterUnits::UseYBOrHigher,
        ]
        .into_iter()
        .collect();
        custom_formatter.includes_count = false;
        custom_formatter.includes_unit = false;

        assert_eq!(custom_formatter.string_from_byte_count(0), "");
        assert_eq!(custom_formatter.string_from_byte_count(1023), "");
        assert_eq!(custom_formatter.string_from_byte_count(1048576), "");
        assert_eq!(custom_formatter.string_from_byte_count(1073741824), "");
        assert_eq!(custom_formatter.string_from_byte_count(1099511627776), "");
        assert_eq!(
            custom_formatter.string_from_byte_count(1237940039285380274899124224),
            ""
        );
    }

    #[test]
    fn test_allowed_units() {
        let mut formatter = ByteCountFormatter::new();

        // Test with default settings
        assert_eq!(formatter.allowed_units, vec![].into_iter().collect());

        // Test after setting custom units
        formatter.allowed_units = vec![
            ByteCountFormatterUnits::UseKB,
            ByteCountFormatterUnits::UseMB,
            ByteCountFormatterUnits::UseGB,
        ]
        .into_iter()
        .collect();

        assert_eq!(
            formatter.allowed_units,
            vec![
                ByteCountFormatterUnits::UseKB,
                ByteCountFormatterUnits::UseMB,
                ByteCountFormatterUnits::UseGB,
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_includes_unit() {
        let mut formatter = ByteCountFormatter::new();

        // Test with default settings
        assert!(formatter.includes_unit);

        // Test after setting custom includes unit value
        formatter.includes_unit = false;
        assert!(!formatter.includes_unit);
    }

    #[test]
    fn test_includes_count() {
        let mut formatter = ByteCountFormatter::new();

        // Test with default settings
        assert!(formatter.includes_count);

        // Test after setting custom includes count value
        formatter.includes_count = false;
        assert!(!formatter.includes_count);
    }
}
