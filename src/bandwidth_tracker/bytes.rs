use std::{
    fmt::Display,
    ops::{Add, Sub},
    time::Duration,
};

use byte_unit::Byte;

#[derive(Clone, Copy, Default, Debug)]
pub struct NumberOfBytes(i32);

#[derive(Clone, Copy, Default)]
pub struct BytesPerSecond(f64);

impl From<i32> for NumberOfBytes {
    fn from(value: i32) -> Self {
        NumberOfBytes(value)
    }
}

impl From<u64> for NumberOfBytes {
    fn from(value: u64) -> Self {
        NumberOfBytes(value as i32)
    }
}

impl Add for NumberOfBytes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        NumberOfBytes(self.0 + rhs.0)
    }
}

impl Sub for NumberOfBytes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        NumberOfBytes(self.0 - rhs.0)
    }
}

impl Display for NumberOfBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Byte::from_u64(self.0 as u64).get_appropriate_unit(byte_unit::UnitType::Decimal)
        )
    }
}

impl BytesPerSecond {
    pub fn new(bytes: NumberOfBytes, duration: Duration) -> BytesPerSecond {
        let duration_as_millis = duration.as_millis();

        if duration_as_millis == 0 {
            BytesPerSecond(0.0)
        } else {
            BytesPerSecond(bytes.0 as f64 / duration_as_millis as f64 * 1000.0)
        }
    }
}

impl Display for BytesPerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/s",
            Byte::from_f64(self.0)
                .unwrap_or_default()
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
        )
    }
}

impl From<NumberOfBytes> for f64 {
    fn from(value: NumberOfBytes) -> Self {
        value.0 as f64
    }
}

impl From<BytesPerSecond> for f64 {
    fn from(value: BytesPerSecond) -> Self {
        value.0
    }
}
