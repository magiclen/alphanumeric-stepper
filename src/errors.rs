use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Error for AlphanumericStepperBuildError {}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Error for AlphanumericStepperEncodeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Error for AlphanumericStepperDecodeError {}

#[cfg(feature = "std")]
#[derive(Debug)]
pub enum AlphanumericStepperEncodeWriteError {
    NumberOutOfRange,
    IOError(std::io::Error),
}

#[cfg(feature = "std")]
impl Display for AlphanumericStepperEncodeWriteError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberOutOfRange => f.write_str("number out of range"),
            Self::IOError(error) => Display::fmt(error, f),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for AlphanumericStepperEncodeWriteError {
    #[inline]
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

#[cfg(feature = "std")]
impl From<AlphanumericStepperEncodeError> for AlphanumericStepperEncodeWriteError {
    #[inline]
    fn from(error: AlphanumericStepperEncodeError) -> Self {
        match error {
            AlphanumericStepperEncodeError::NumberOutOfRange => Self::NumberOutOfRange,
        }
    }
}

#[cfg(feature = "std")]
impl Error for AlphanumericStepperEncodeWriteError {}
