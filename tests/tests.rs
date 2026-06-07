#[cfg(feature = "std")]
use std::io;

#[cfg(feature = "std")]
use alphanumeric_stepper::AlphanumericStepperEncodeWriteError;
use alphanumeric_stepper::{
    AlphanumericStepper, AlphanumericStepperBuildError, AlphanumericStepperDecodeError,
    AlphanumericStepperEncodeError,
};

fn assert_u16_code(stepper: &AlphanumericStepper<u16>, number: u16, encoded: &str) {
    assert_eq!(encoded, stepper.encode(number).unwrap());
    assert_eq!(number, stepper.decode(encoded).unwrap());
}

fn assert_round_trip_u16(width: usize) {
    let stepper = AlphanumericStepper::<u16>::new(width).unwrap();

    for number in 0..=stepper.max_number() {
        let encoded = stepper.encode(number).unwrap();

        assert_eq!(width, encoded.len());
        assert_eq!(number, stepper.decode(&encoded).unwrap());
    }
}

#[test]
fn new_validates_representative_backend_widths() {
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

    assert_eq!(6, AlphanumericStepper::<u32>::new(6).unwrap().width());
    assert!(matches!(
        AlphanumericStepper::<u32>::new(7),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    assert_eq!(13, AlphanumericStepper::<u64>::new(13).unwrap().width());
    assert!(matches!(
        AlphanumericStepper::<u64>::new(14),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    assert_eq!(27, AlphanumericStepper::<u128>::new(27).unwrap().width());
    assert!(matches!(
        AlphanumericStepper::<u128>::new(28),
        Err(AlphanumericStepperBuildError::InvalidWidth)
    ));

    assert_eq!(3, AlphanumericStepper::<usize>::new(3).unwrap().width());

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
fn encode_and_decode_follow_sequence_boundaries() {
    let width_1 = AlphanumericStepper::<u8>::new(1).unwrap();

    assert_eq!("0", width_1.encode(0).unwrap());
    assert_eq!("9", width_1.encode(9).unwrap());
    assert_eq!("A", width_1.encode(10).unwrap());
    assert_eq!("Z", width_1.encode(35).unwrap());

    let width_3 = AlphanumericStepper::<u16>::new(3).unwrap();

    for (number, encoded) in [
        (0, "000"),
        (999, "999"),
        (1000, "A00"),
        (3599, "Z99"),
        (3600, "AA0"),
        (10_359, "ZZ9"),
        (10_360, "AAA"),
        (27_935, "ZZZ"),
    ] {
        assert_u16_code(&width_3, number, encoded);
    }
}

#[test]
fn encode_and_decode_round_trip_small_ranges() {
    let width_1 = AlphanumericStepper::<u8>::new(1).unwrap();

    for number in 0..=width_1.max_number() {
        let encoded = width_1.encode(number).unwrap();

        assert_eq!(1, encoded.len());
        assert_eq!(number, width_1.decode(&encoded).unwrap());
    }

    for width in 1..=3 {
        assert_round_trip_u16(width);
    }
}

#[test]
fn encode_can_append_to_reusable_outputs() {
    let stepper = AlphanumericStepper::<u16>::new(3).unwrap();

    let mut s = String::from("prefix:");
    stepper.encode_to_string(1000, &mut s).unwrap();
    assert_eq!("prefix:A00", s);

    let mut bytes = b"prefix:".to_vec();
    stepper.encode_to_vec(3599, &mut bytes).unwrap();
    assert_eq!(b"prefix:Z99", bytes.as_slice());

    #[cfg(feature = "std")]
    {
        let mut writer = b"prefix:".to_vec();
        stepper.encode_to_writer(10_360, &mut writer).unwrap();
        assert_eq!(b"prefix:AAA", writer.as_slice());
    }
}

#[test]
fn encode_rejects_out_of_range_without_touching_outputs() {
    let stepper = AlphanumericStepper::<u16>::new(3).unwrap();
    let number = stepper.max_number() + 1;

    assert!(matches!(
        stepper.encode(number),
        Err(AlphanumericStepperEncodeError::NumberOutOfRange)
    ));

    let mut s = String::from("keep");
    assert!(matches!(
        stepper.encode_to_string(number, &mut s),
        Err(AlphanumericStepperEncodeError::NumberOutOfRange)
    ));
    assert_eq!("keep", s);

    let mut bytes = b"keep".to_vec();
    assert!(matches!(
        stepper.encode_to_vec(number, &mut bytes),
        Err(AlphanumericStepperEncodeError::NumberOutOfRange)
    ));
    assert_eq!(b"keep", bytes.as_slice());

    #[cfg(feature = "std")]
    {
        let mut writer = b"keep".to_vec();
        assert!(matches!(
            stepper.encode_to_writer(number, &mut writer),
            Err(AlphanumericStepperEncodeWriteError::NumberOutOfRange)
        ));
        assert_eq!(b"keep", writer.as_slice());
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_to_writer_reports_writer_errors() {
    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let stepper = AlphanumericStepper::<u16>::new(3).unwrap();
    let mut writer = FailingWriter;

    assert!(matches!(
        stepper.encode_to_writer(0, &mut writer),
        Err(AlphanumericStepperEncodeWriteError::IOError(_))
    ));
}

#[test]
fn decode_rejects_basic_invalid_inputs() {
    let stepper = AlphanumericStepper::<u16>::new(3).unwrap();

    assert!(matches!(stepper.decode("12"), Err(AlphanumericStepperDecodeError::InvalidLength)));

    for input in ["a00", "0A0", "AA-"] {
        assert!(matches!(
            stepper.decode(input),
            Err(AlphanumericStepperDecodeError::InvalidCharacter)
        ));
    }
}
