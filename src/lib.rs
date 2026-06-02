/*!
# Alphanumeric Stepper

A reversible alphanumeric sequence codec for compact serial codes like 000..999, A00..Z99, AA0..ZZ9, and AAA..ZZZ.

## Examples

Encode a number to an Alphanumeric Stepper string.

```rust
use alphanumeric_stepper::AlphanumericStepper;

let stepper = AlphanumericStepper::<u16>::new(3).unwrap();

assert_eq!("000", stepper.encode(0).unwrap());
assert_eq!("001", stepper.encode(1).unwrap());
assert_eq!("999", stepper.encode(999).unwrap());
assert_eq!("A00", stepper.encode(1000).unwrap());
assert_eq!("A99", stepper.encode(1099).unwrap());
assert_eq!("B00", stepper.encode(1100).unwrap());
assert_eq!("ZZZ", stepper.encode(27935).unwrap());
```

Decode an Alphanumeric Stepper string to a number.

```rust
use alphanumeric_stepper::AlphanumericStepper;

let stepper = AlphanumericStepper::<u16>::new(3).unwrap();

assert_eq!(0, stepper.decode("000").unwrap());
assert_eq!(1, stepper.decode("001").unwrap());
assert_eq!(999, stepper.decode("999").unwrap());
assert_eq!(1000, stepper.decode("A00").unwrap());
assert_eq!(1099, stepper.decode("A99").unwrap());
assert_eq!(1100, stepper.decode("B00").unwrap());
assert_eq!(27935, stepper.decode("ZZZ").unwrap());
```

## Std

Enable the `std` features to compile this crate with std.

```toml
[dependencies.alphanumeric-stepper]
version = "*"
features = ["std"]
```
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub use errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphanumericStepper<T = u128> {
    width:       usize,
    max_numbers: Vec<T>,
}

impl<T> AlphanumericStepper<T> {
    /// Returns the width.
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }
}

impl<T: Copy> AlphanumericStepper<T> {
    /// Returns the maximum number that can be encoded.
    #[inline]
    pub fn max_number(&self) -> T {
        self.max_numbers[self.width]
    }
}

macro_rules! impl_alphanumeric_stepper_backend {
    ($($integer:ty),+ $(,)?) => {
        $(
            impl AlphanumericStepper<$integer> {
                #[inline]
                fn checked_pow(base: $integer, exp: usize) -> Option<$integer> {
                    let mut value: $integer = 1;

                    for _ in 0..exp {
                        value = value.checked_mul(base)?;
                    }

                    Some(value)
                }

                /// Creates a new `AlphanumericStepper` instance with the specified width.
                #[inline]
                pub fn new(width: usize) -> Result<Self, AlphanumericStepperBuildError> {
                    if width == 0 {
                        return Err(AlphanumericStepperBuildError::InvalidWidth);
                    }

                    let mut max_numbers = Vec::with_capacity(width + 1);
                    let mut sum: $integer = 0;
                    let mut term = Self::checked_pow(10, width)
                        .ok_or(AlphanumericStepperBuildError::InvalidWidth)?;

                    for alphabet_count in 0..=width {
                        sum = sum
                            .checked_add(term)
                            .ok_or(AlphanumericStepperBuildError::InvalidWidth)?;

                        max_numbers.push(sum - 1);

                        if alphabet_count < width {
                            term = (term / 10)
                                .checked_mul(26)
                                .ok_or(AlphanumericStepperBuildError::InvalidWidth)?;
                        }
                    }

                    Ok(Self {
                        width,
                        max_numbers,
                    })
                }

                /// Encodes a number into a string.
                pub fn encode(&self, number: $integer) -> Result<String, AlphanumericStepperEncodeError> {
                    if number > self.max_number() {
                        return Err(AlphanumericStepperEncodeError::NumberOutOfRange);
                    }

                    let mut s = String::with_capacity(self.width);

                    if number <= self.max_numbers[0] {
                        s.write_fmt(format_args!("{:0>width$}", number, width = self.width)).unwrap();
                    } else {
                        for (alphabet_count, max_number) in
                            self.max_numbers.iter().copied().enumerate().skip(1)
                        {
                            if number > max_number {
                                continue;
                            }

                            let digit_count = self.width - alphabet_count;
                            let mut n = number - self.max_numbers[alphabet_count - 1] - 1;
                            let digit_base = Self::checked_pow(10, digit_count).unwrap();
                            let mut p = digit_base
                                .checked_mul(Self::checked_pow(26, alphabet_count - 1).unwrap())
                                .unwrap();

                            for _ in 0..alphabet_count {
                                let d = n / p;

                                s.push((b'A' + d as u8) as char);

                                n -= d * p;
                                p /= 26;
                            }

                            if digit_count > 0 {
                                s.write_fmt(format_args!("{:0>width$}", n, width = digit_count)).unwrap();
                            }

                            break;
                        }
                    }

                    Ok(s)
                }

                /// Decodes a string into a number.
                pub fn decode(&self, s: impl AsRef<str>) -> Result<$integer, AlphanumericStepperDecodeError> {
                    let s = s.as_ref();

                    if s.len() != self.width {
                        return Err(AlphanumericStepperDecodeError::InvalidLength);
                    }

                    let bytes = s.as_bytes();
                    let mut alphabet_count = 0;

                    while alphabet_count < bytes.len() {
                        let b = bytes[alphabet_count];

                        if b.is_ascii_uppercase() {
                            alphabet_count += 1;
                        } else if b.is_ascii_digit() {
                            break;
                        } else {
                            return Err(AlphanumericStepperDecodeError::InvalidCharacter);
                        }
                    }

                    for &b in &bytes[alphabet_count..] {
                        if !b.is_ascii_digit() {
                            return Err(AlphanumericStepperDecodeError::InvalidCharacter);
                        }
                    }

                    let digit_count = self.width - alphabet_count;
                    let mut number = if alphabet_count == 0 {
                        0
                    } else {
                        self.max_numbers[alphabet_count - 1] + 1
                    };

                    let mut n: $integer = 0;

                    for &b in &bytes[..alphabet_count] {
                        n = n
                            .checked_mul(26)
                            .and_then(|value| value.checked_add((b - b'A') as $integer))
                            .unwrap();
                    }

                    number = number
                        .checked_add(n.checked_mul(Self::checked_pow(10, digit_count).unwrap()).unwrap())
                        .unwrap();

                    n = 0;

                    for &b in &bytes[alphabet_count..] {
                        n = n
                            .checked_mul(10)
                            .and_then(|value| value.checked_add((b - b'0') as $integer))
                            .unwrap();
                    }

                    number = number.checked_add(n).unwrap();

                    Ok(number)
                }
            }
        )+
    };
}

impl_alphanumeric_stepper_backend!(u8, u16, u32, u64, u128, usize);
