use std::{
    fmt::Display,
    ops::{Add, Sub},
    time::Duration,
};

use byte_unit::Byte;

#[derive(Clone, Copy, Default)]
pub struct NumberOfBytes(i32);

#[derive(Clone, Copy, Default)]
pub struct BytesPerSecond(u64);

impl From<i32> for NumberOfBytes {
    fn from(value: i32) -> Self {
        NumberOfBytes(value)
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
            Byte::from_bytes(self.0 as u128).get_appropriate_unit(false)
        )
    }
}

impl BytesPerSecond {
    pub fn new(bytes: NumberOfBytes, duration: Duration) -> BytesPerSecond {
        BytesPerSecond(bytes.0 as u64 / duration.as_secs())
    }
}

impl Display for BytesPerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/s",
            Byte::from_bytes(self.0 as u128).get_appropriate_unit(false)
        )
    }
}
