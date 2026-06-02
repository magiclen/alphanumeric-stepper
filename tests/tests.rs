use alphanumeric_stepper::{
    AlphanumericStepper, AlphanumericStepperBuildError, AlphanumericStepperDecodeError,
    AlphanumericStepperEncodeError,
};

macro_rules! assert_boundary_cases {
    ($ty:ty, $width:expr, [$(($number:expr, $encoded:literal)),+ $(,)?]) => {{
        let stepper = AlphanumericStepper::<$ty>::new($width).unwrap();

        for (number, encoded) in [$(($number as $ty, $encoded)),+] {
            assert_eq!(encoded, stepper.encode(number).unwrap());
            assert_eq!(number, stepper.decode(encoded).unwrap());
        }
    }};
}

macro_rules! assert_round_trip_all_values {
    ($ty:ty, $($width:expr),+ $(,)?) => {{
        $(
            let stepper = AlphanumericStepper::<$ty>::new($width).unwrap();

            for number in 0..=stepper.max_number() {
                let encoded = stepper.encode(number).unwrap();

                assert_eq!($width, encoded.len());
                assert_eq!(number, stepper.decode(&encoded).unwrap());
            }
        )+
    }};
}

#[test]
fn new_validates_width_against_backend_capacity() {
    assert!(matches!(
        AlphanumericStepper::<u8>::new(0),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    let width_1 = AlphanumericStepper::<u8>::new(1).unwrap();

    assert_eq!(1, width_1.width());
    assert_eq!(35_u8, width_1.max_number());
    assert!(matches!(
        AlphanumericStepper::<u8>::new(2),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    let width_3 = AlphanumericStepper::<u16>::new(3).unwrap();

    assert_eq!(3, width_3.width());
    assert_eq!(27_935_u16, width_3.max_number());
    assert!(matches!(
        AlphanumericStepper::<u16>::new(4),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    let width_6 = AlphanumericStepper::<u32>::new(6).unwrap();

    assert_eq!(6, width_6.width());
    assert!(matches!(
        AlphanumericStepper::<u32>::new(7),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    let width_13 = AlphanumericStepper::<u64>::new(13).unwrap();

    assert_eq!(13, width_13.width());
    assert!(matches!(
        AlphanumericStepper::<u64>::new(14),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    let width_27 = AlphanumericStepper::<u128>::new(27).unwrap();

    assert_eq!(27, width_27.width());
    assert!(matches!(
        AlphanumericStepper::<u128>::new(28),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    let mut max_supported_width = 0;

    for width in 1.. {
        match AlphanumericStepper::<usize>::new(width) {
            Ok(stepper) => {
                max_supported_width = width;
                assert_eq!(width, stepper.width());
            },
            Err(AlphanumericStepperBuildError::InvalidWidth) => {
                assert!(max_supported_width >= 3);
                assert_eq!(max_supported_width + 1, width);
                break;
            },
        }
    }
}

#[test]
fn encode_and_decode_match_boundary_examples_for_all_backends() {
    assert_boundary_cases!(u8, 1, [(0, "0"), (9, "9"), (10, "A"), (35, "Z")]);

    assert_boundary_cases!(u16, 3, [
        (0, "000"),
        (1, "001"),
        (999, "999"),
        (1000, "A00"),
        (1099, "A99"),
        (1100, "B00"),
        (3599, "Z99"),
        (3600, "AA0"),
        (3609, "AA9"),
        (3610, "AB0"),
        (10359, "ZZ9"),
        (10360, "AAA"),
        (27935, "ZZZ"),
    ]);

    assert_boundary_cases!(u32, 3, [
        (0, "000"),
        (1, "001"),
        (999, "999"),
        (1000, "A00"),
        (1099, "A99"),
        (1100, "B00"),
        (3599, "Z99"),
        (3600, "AA0"),
        (3609, "AA9"),
        (3610, "AB0"),
        (10359, "ZZ9"),
        (10360, "AAA"),
        (27935, "ZZZ"),
    ]);

    assert_boundary_cases!(u64, 3, [
        (0, "000"),
        (1, "001"),
        (999, "999"),
        (1000, "A00"),
        (1099, "A99"),
        (1100, "B00"),
        (3599, "Z99"),
        (3600, "AA0"),
        (3609, "AA9"),
        (3610, "AB0"),
        (10359, "ZZ9"),
        (10360, "AAA"),
        (27935, "ZZZ"),
    ]);

    assert_boundary_cases!(u128, 3, [
        (0, "000"),
        (1, "001"),
        (999, "999"),
        (1000, "A00"),
        (1099, "A99"),
        (1100, "B00"),
        (3599, "Z99"),
        (3600, "AA0"),
        (3609, "AA9"),
        (3610, "AB0"),
        (10359, "ZZ9"),
        (10360, "AAA"),
        (27935, "ZZZ"),
    ]);

    assert_boundary_cases!(usize, 3, [
        (0, "000"),
        (1, "001"),
        (999, "999"),
        (1000, "A00"),
        (1099, "A99"),
        (1100, "B00"),
        (3599, "Z99"),
        (3600, "AA0"),
        (3609, "AA9"),
        (3610, "AB0"),
        (10359, "ZZ9"),
        (10360, "AAA"),
        (27935, "ZZZ"),
    ]);
}

#[test]
fn encode_and_decode_round_trip_for_supported_backends() {
    assert_round_trip_all_values!(u8, 1);
    assert_round_trip_all_values!(u16, 1, 2, 3);
    assert_round_trip_all_values!(u32, 1, 2, 3);
    assert_round_trip_all_values!(u64, 1, 2, 3);
    assert_round_trip_all_values!(u128, 1, 2, 3);
    assert_round_trip_all_values!(usize, 1, 2, 3);
}

#[test]
fn encode_rejects_numbers_above_the_supported_range() {
    let stepper = AlphanumericStepper::<u16>::new(3).unwrap();

    assert!(matches!(
        stepper.encode(stepper.max_number() + 1),
        Err(AlphanumericStepperEncodeError::NumberOutOfRange)
    ));
}

#[test]
fn decode_rejects_invalid_length_and_non_canonical_formats() {
    let stepper = AlphanumericStepper::<u16>::new(3).unwrap();

    assert!(matches!(stepper.decode("12"), Err(AlphanumericStepperDecodeError::InvalidLength)));

    for input in ["a00", "0A0", "A0A", "AA-"] {
        assert!(matches!(
            stepper.decode(input),
            Err(AlphanumericStepperDecodeError::InvalidCharacter)
        ));
    }
}
