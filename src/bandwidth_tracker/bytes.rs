use std::{
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

impl ToString for NumberOfBytes {
    fn to_string(&self) -> String {
        format!(
            "{}",
            Byte::from_bytes(self.0 as u128).get_appropriate_unit(true)
        )
    }
}

impl BytesPerSecond {
    pub fn new(bytes: NumberOfBytes, duration: Duration) -> BytesPerSecond {
        BytesPerSecond(bytes.0 as u64 / duration.as_secs())
    }
}

impl ToString for BytesPerSecond {
    fn to_string(&self) -> String {
        format!(
            "{}/s",
            Byte::from_bytes(self.0 as u128).get_appropriate_unit(true)
        )
    }
}
