use crate::NumericValue;

pub trait PrimitiveNumber: Copy {
    fn from_numeric(n: &NumericValue) -> Self
    where
        Self: Sized;
}

impl PrimitiveNumber for i8 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i,
            NumericValue::Short(i) => *i as i8,
            NumericValue::Int(i) => *i as i8,
            NumericValue::Long(i) => *i as i8,
            NumericValue::UChar(i) => *i as i8,
            NumericValue::UShort(i) => *i as i8,
            NumericValue::UInt(i) => *i as i8,
            NumericValue::ULong(i) => *i as i8,
            NumericValue::Float(i) => *i as i8,
            NumericValue::Double(i) => *i as i8,
        }
    }

    
}

impl PrimitiveNumber for u8 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as u8,
            NumericValue::Short(i) => *i as u8,
            NumericValue::Int(i) => *i as u8,
            NumericValue::Long(i) => *i as u8,
            NumericValue::UChar(i) => *i,
            NumericValue::UShort(i) => *i as u8,
            NumericValue::UInt(i) => *i as u8,
            NumericValue::ULong(i) => *i as u8,
            NumericValue::Float(i) => *i as u8,
            NumericValue::Double(i) => *i as u8,
        }
    }
}

impl PrimitiveNumber for i16 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as i16,
            NumericValue::Short(i) => *i,
            NumericValue::Int(i) => *i as i16,
            NumericValue::Long(i) => *i as i16,
            NumericValue::UChar(i) => *i as i16,
            NumericValue::UShort(i) => *i as i16,
            NumericValue::UInt(i) => *i as i16,
            NumericValue::ULong(i) => *i as i16,
            NumericValue::Float(i) => *i as i16,
            NumericValue::Double(i) => *i as i16,
        }
    }
}

impl PrimitiveNumber for u16 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as u16,
            NumericValue::Short(i) => *i as u16,
            NumericValue::Int(i) => *i as u16,
            NumericValue::Long(i) => *i as u16,
            NumericValue::UChar(i) => *i as u16,
            NumericValue::UShort(i) => *i,
            NumericValue::UInt(i) => *i as u16,
            NumericValue::ULong(i) => *i as u16,
            NumericValue::Float(i) => *i as u16,
            NumericValue::Double(i) => *i as u16,
        }
    }
}

impl PrimitiveNumber for i32 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as i32,
            NumericValue::Short(i) => *i as i32,
            NumericValue::Int(i) => *i,
            NumericValue::Long(i) => *i as i32,
            NumericValue::UChar(i) => *i as i32,
            NumericValue::UShort(i) => *i as i32,
            NumericValue::UInt(i) => *i as i32,
            NumericValue::ULong(i) => *i as i32,
            NumericValue::Float(i) => *i as i32,
            NumericValue::Double(i) => *i as i32,
        }
    }
}

impl PrimitiveNumber for u32 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as u32,
            NumericValue::Short(i) => *i as u32,
            NumericValue::Int(i) => *i as u32,
            NumericValue::Long(i) => *i as u32,
            NumericValue::UChar(i) => *i as u32,
            NumericValue::UShort(i) => *i as u32,
            NumericValue::UInt(i) => *i,
            NumericValue::ULong(i) => *i as u32,
            NumericValue::Float(i) => *i as u32,
            NumericValue::Double(i) => *i as u32,
        }
    }
}

impl PrimitiveNumber for i64 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as i64,
            NumericValue::Short(i) => *i as i64,
            NumericValue::Int(i) => *i as i64,
            NumericValue::Long(i) => *i,
            NumericValue::UChar(i) => *i as i64,
            NumericValue::UShort(i) => *i as i64,
            NumericValue::UInt(i) => *i as i64,
            NumericValue::ULong(i) => *i as i64,
            NumericValue::Float(i) => *i as i64,
            NumericValue::Double(i) => *i as i64,
        }
    }
}

impl PrimitiveNumber for u64 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as u64,
            NumericValue::Short(i) => *i as u64,
            NumericValue::Int(i) => *i as u64,
            NumericValue::Long(i) => *i as u64,
            NumericValue::UChar(i) => *i as u64,
            NumericValue::UShort(i) => *i as u64,
            NumericValue::UInt(i) => *i as u64,
            NumericValue::ULong(i) => *i,
            NumericValue::Float(i) => *i as u64,
            NumericValue::Double(i) => *i as u64,
        }
    }
}

impl PrimitiveNumber for f32 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as f32,
            NumericValue::Short(i) => *i as f32,
            NumericValue::Int(i) => *i as f32,
            NumericValue::Long(i) => *i as f32,
            NumericValue::UChar(i) => *i as f32,
            NumericValue::UShort(i) => *i as f32,
            NumericValue::UInt(i) => *i as f32,
            NumericValue::ULong(i) => *i as f32,
            NumericValue::Float(i) => *i,
            NumericValue::Double(i) => *i as f32,
        }
    }
}

impl PrimitiveNumber for f64 {
    fn from_numeric(n: &NumericValue) -> Self {
        match n {
            NumericValue::Char(i) => *i as f64,
            NumericValue::Short(i) => *i as f64,
            NumericValue::Int(i) => *i as f64,
            NumericValue::Long(i) => *i as f64,
            NumericValue::UChar(i) => *i as f64,
            NumericValue::UShort(i) => *i as f64,
            NumericValue::UInt(i) => *i as f64,
            NumericValue::ULong(i) => *i as f64,
            NumericValue::Float(i) => *i as f64,
            NumericValue::Double(i) => *i,
        }
    }
}
