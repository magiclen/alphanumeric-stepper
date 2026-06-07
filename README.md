Alphanumeric Stepper
====================

[![CI](https://github.com/magiclen/alphanumeric-stepper/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/alphanumeric-stepper/actions/workflows/ci.yml)

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

## Crates.io

https://crates.io/crates/alphanumeric-stepper

## Documentation

https://docs.rs/alphanumeric-stepper

## License

[MIT](LICENSE)