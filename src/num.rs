use core::{fmt, str::FromStr};

use alloc::string::String;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Number {
    Bool(bool),
    Int(isize),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    UInt(usize),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    Float(f32),
    Double(f64),
}

impl Number {
    #[must_use]
    pub fn bool(&self) -> bool {
        match self {
            Number::Bool(value) => *value,
            Number::Int(value) => *value != 0,
            Number::Int8(value) => *value != 0,
            Number::Int16(value) => *value != 0,
            Number::Int32(value) => *value != 0,
            Number::UInt(value) => *value != 0,
            Number::UInt8(value) => *value != 0,
            Number::UInt16(value) => *value != 0,
            Number::UInt32(value) => *value != 0,
            Number::Float(value) => *value != 0.0,
            Number::Double(value) => *value != 0.0,
        }
    }

    #[must_use]
    pub fn int(&self) -> isize {
        match self {
            Number::Bool(value) => isize::from(*value),
            Number::Int(value) => *value,
            Number::Int8(value) => *value as isize,
            Number::Int16(value) => *value as isize,
            Number::Int32(value) => *value as isize,
            Number::UInt(value) => *value as isize,
            Number::UInt8(value) => *value as isize,
            Number::UInt16(value) => *value as isize,
            Number::UInt32(value) => *value as isize,
            Number::Float(value) => *value as isize,
            Number::Double(value) => *value as isize,
        }
    }

    #[must_use]
    pub fn int8(&self) -> i8 {
        match self {
            Number::Bool(value) => *value as i8,
            Number::Int(value) => *value as i8,
            Number::Int8(value) => *value,
            Number::Int16(value) => *value as i8,
            Number::Int32(value) => *value as i8,
            Number::UInt(value) => *value as i8,
            Number::UInt8(value) => *value as i8,
            Number::UInt16(value) => *value as i8,
            Number::UInt32(value) => *value as i8,
            Number::Float(value) => *value as i8,
            Number::Double(value) => *value as i8,
        }
    }

    #[must_use]
    pub fn int16(&self) -> i16 {
        match self {
            Number::Bool(value) => *value as i16,
            Number::Int(value) => *value as i16,
            Number::Int8(value) => *value as i16,
            Number::Int16(value) => *value,
            Number::Int32(value) => *value as i16,
            Number::UInt(value) => *value as i16,
            Number::UInt8(value) => *value as i16,
            Number::UInt16(value) => *value as i16,
            Number::UInt32(value) => *value as i16,
            Number::Float(value) => *value as i16,
            Number::Double(value) => *value as i16,
        }
    }

    #[must_use]
    pub fn int32(&self) -> i32 {
        match self {
            Number::Bool(value) => *value as i32,
            Number::Int(value) => *value as i32,
            Number::Int8(value) => *value as i32,
            Number::Int16(value) => *value as i32,
            Number::Int32(value) => *value,
            Number::UInt(value) => *value as i32,
            Number::UInt8(value) => *value as i32,
            Number::UInt16(value) => *value as i32,
            Number::UInt32(value) => *value as i32,
            Number::Float(value) => *value as i32,
            Number::Double(value) => *value as i32,
        }
    }

    #[must_use]
    pub fn uint(&self) -> usize {
        match self {
            Number::Bool(value) => *value as usize,
            Number::Int(value) => *value as usize,
            Number::Int8(value) => *value as usize,
            Number::Int16(value) => *value as usize,
            Number::Int32(value) => *value as usize,
            Number::UInt(value) => *value,
            Number::UInt8(value) => *value as usize,
            Number::UInt16(value) => *value as usize,
            Number::UInt32(value) => *value as usize,
            Number::Float(value) => *value as usize,
            Number::Double(value) => *value as usize,
        }
    }

    #[must_use]
    pub fn uint8(&self) -> u8 {
        match self {
            Number::Bool(value) => *value as u8,
            Number::Int(value) => *value as u8,
            Number::Int8(value) => *value as u8,
            Number::Int16(value) => *value as u8,
            Number::Int32(value) => *value as u8,
            Number::UInt(value) => *value as u8,
            Number::UInt8(value) => *value,
            Number::UInt16(value) => *value as u8,
            Number::UInt32(value) => *value as u8,
            Number::Float(value) => *value as u8,
            Number::Double(value) => *value as u8,
        }
    }

    #[must_use]
    pub fn uint16(&self) -> u16 {
        match self {
            Number::Bool(value) => *value as u16,
            Number::Int(value) => *value as u16,
            Number::Int8(value) => *value as u16,
            Number::Int16(value) => *value as u16,
            Number::Int32(value) => *value as u16,
            Number::UInt(value) => *value as u16,
            Number::UInt8(value) => *value as u16,
            Number::UInt16(value) => *value,
            Number::UInt32(value) => *value as u16,
            Number::Float(value) => *value as u16,
            Number::Double(value) => *value as u16,
        }
    }

    #[must_use]
    pub fn uint32(&self) -> u32 {
        match self {
            Number::Bool(value) => *value as u32,
            Number::Int(value) => *value as u32,
            Number::Int8(value) => *value as u32,
            Number::Int16(value) => *value as u32,
            Number::Int32(value) => *value as u32,
            Number::UInt(value) => *value as u32,
            Number::UInt8(value) => *value as u32,
            Number::UInt16(value) => *value as u32,
            Number::UInt32(value) => *value,
            Number::Float(value) => *value as u32,
            Number::Double(value) => *value as u32,
        }
    }

    #[must_use]
    pub fn float(&self) -> f32 {
        match self {
            Number::Bool(value) => *value as u8 as f32,
            Number::Int(value) => *value as f32,
            Number::Int8(value) => *value as f32,
            Number::Int16(value) => *value as f32,
            Number::Int32(value) => *value as f32,
            Number::UInt(value) => *value as f32,
            Number::UInt8(value) => *value as f32,
            Number::UInt16(value) => *value as f32,
            Number::UInt32(value) => *value as f32,
            Number::Float(value) => *value,
            Number::Double(value) => *value as f32,
        }
    }

    #[must_use]
    pub fn double(&self) -> f64 {
        match self {
            Number::Bool(value) => *value as u8 as f64,
            Number::Int(value) => *value as f64,
            Number::Int8(value) => *value as f64,
            Number::Int16(value) => *value as f64,
            Number::Int32(value) => *value as f64,
            Number::UInt(value) => *value as f64,
            Number::UInt8(value) => *value as f64,
            Number::UInt16(value) => *value as f64,
            Number::UInt32(value) => *value as f64,
            Number::Float(value) => *value as f64,
            Number::Double(value) => *value,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Bool(value) => write!(f, "{}", value),
            Number::Int(value) => write!(f, "{}", value),
            Number::Int8(value) => write!(f, "{}", value),
            Number::Int16(value) => write!(f, "{}", value),
            Number::Int32(value) => write!(f, "{}", value),
            Number::UInt(value) => write!(f, "{}", value),
            Number::UInt8(value) => write!(f, "{}", value),
            Number::UInt16(value) => write!(f, "{}", value),
            Number::UInt32(value) => write!(f, "{}", value),
            Number::Float(value) => write!(f, "{}", value),
            Number::Double(value) => write!(f, "{}", value),
        }
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bool_result = s.parse::<bool>().map(Number::Bool);
        let int_result = s.parse::<isize>().map(Number::Int);
        let int8_result = s.parse::<i8>().map(Number::Int8);
        let int16_result = s.parse::<i16>().map(Number::Int16);
        let int32_result = s.parse::<i32>().map(Number::Int32);
        let uint_result = s.parse::<usize>().map(Number::UInt);
        let uint8_result = s.parse::<u8>().map(Number::UInt8);
        let uint16_result = s.parse::<u16>().map(Number::UInt16);
        let uint32_result = s.parse::<u32>().map(Number::UInt32);
        let float_result = s.parse::<f32>().map(Number::Float);
        let double_result = s.parse::<f64>().map(Number::Double);

        bool_result
            .or(int_result)
            .or(int8_result)
            .or(int16_result)
            .or(int32_result)
            .or(uint_result)
            .or(uint8_result)
            .or(uint16_result)
            .or(uint32_result)
            .or(float_result)
            .or(double_result)
            .map_err(|e| alloc::format!("{e}"))
    }
}

impl From<bool> for Number {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Self::Int(value)
    }
}
