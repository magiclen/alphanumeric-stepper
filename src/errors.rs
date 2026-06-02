use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum AlphanumericStepperBuildError {
    InvalidWidth,
}

impl Display for AlphanumericStepperBuildError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWidth => f.write_str("invalid width"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for AlphanumericStepperBuildError {}

#[derive(Debug, Clone)]
pub enum AlphanumericStepperEncodeError {
    NumberOutOfRange,
}

impl Display for AlphanumericStepperEncodeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberOutOfRange => f.write_str("number out of range"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for AlphanumericStepperEncodeError {}

#[derive(Debug, Clone)]
pub enum AlphanumericStepperDecodeError {
    InvalidLength,
    InvalidCharacter,
}

impl Display for AlphanumericStepperDecodeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => f.write_str("invalid length"),
            Self::InvalidCharacter => f.write_str("invalid character"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for AlphanumericStepperDecodeError {}
