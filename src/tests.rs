use crate::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use failure::Fail;
use mocks::*;

// Mock implementations.
// Note: mockall crate does not help in no_std environment (yet)
//       so we do it the manually
mod mocks {
    use super::*;

    #[derive(Debug, Fail)]
    #[fail(display = "An error occurred")]
    pub struct MockedOutputPinError;

    fn set_bit(input: u128, bit: u16) -> u128 {
        let mask: u128 = 1_u128 << bit;
        input | mask
    }

    fn reset_bit(input: u128, bit: u16) -> u128 {
        let mask: u128 = !(1_u128 << bit);
        input & mask
    }

    #[test]
    fn set_bit_test() {
        assert_eq!(0b11, set_bit(0b10, 0));
        assert_eq!(0b11, set_bit(0b01, 1));
    }

    #[test]
    fn reset_bit_test() {
        assert_eq!(0b10, reset_bit(0b11, 0));
        assert_eq!(0b01, reset_bit(0b11, 1));
    }

    /// A mock implementation of `OutputPin`.
    pub struct MockedOutputPin {
        /// The state of the pin.
        collected_states: u128,
        collected_no_of_calls: u16,
        expected_states: u128,
        expected_no_of_calls: u16,
    }

    impl MockedOutputPin {
        pub fn expected(expected_no_of_calls: u16, expected_states: u128) -> Self {
            Self {
                collected_states: 0_u128,
                collected_no_of_calls: 0_u16,
                expected_states,
                expected_no_of_calls,
            }
        }
    }
    impl OutputPin for MockedOutputPin {
        type Error = MockedOutputPinError;

        fn set_high(&mut self) -> Result<(), MockedOutputPinError> {
            self.collected_states = set_bit(self.collected_states, self.collected_no_of_calls);
            self.collected_no_of_calls += 1;
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), MockedOutputPinError> {
            self.collected_states = reset_bit(self.collected_states, self.collected_no_of_calls);
            self.collected_no_of_calls += 1;
            Ok(())
        }
    }

    impl Drop for MockedOutputPin {
        fn drop(&mut self) {
            // Evaluation happens at the destructor (automatically)
            assert_eq!(self.expected_no_of_calls, self.collected_no_of_calls);
            assert_eq!(self.expected_states, self.collected_states);
        }
    }
}

mod fn_state_at_position {
    use super::super::state_at_position;

    #[test]
    fn at_zero_pos() {
        assert!(!state_at_position(0b110, 0));
        assert!(state_at_position(0b101, 0));
    }

    #[test]
    fn at_first_pos() {
        assert!(!state_at_position(0b101, 1));
        assert!(state_at_position(0b010, 1));
    }
    #[test]
    fn max_first_pos() {
        assert!(!state_at_position(0, 127));
        assert!(state_at_position(!0, 127));
    }
}

mod on_off_sequence_output {
    use super::*;

    mod set {
        use super::*;

        #[test]
        #[should_panic]
        fn too_many_states() {
            let pin_mock = MockedOutputPin::expected(0, 0b0_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            ledout.set(1, 128, Repeat::Never);
        }

        #[test]
        #[should_panic]
        fn no_zero_sequence_length() {
            let pin_mock = MockedOutputPin::expected(0, 0b0_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            ledout.set(1, 0, Repeat::Never);
        }
    }

    mod update_scaling {
        use super::*;

        #[test]
        fn no_update_at_scale_four() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(0, 0b0_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            for _index in 0..2 {
                ledout.update()?;
            }
            Ok(())
        }

        #[test]
        fn one_update_at_scale_four() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(1, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            ledout.set(0b1, 1, Repeat::Never);
            for _index in 0..4 {
                ledout.update()?;
            }
            Ok(())
        }

        #[test]
        fn three_updates_at_scale_four() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(3, 0b101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            ledout.set(0b101, 3, Repeat::Never);
            for _index in 0..12 {
                ledout.update()?;
            }
            Ok(())
        }

        #[test]
        fn one_update_at_scale_one() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(1, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 1, Repeat::Never);
            ledout.update()?;
            Ok(())
        }

        #[test]
        fn three_updates_at_scale_one() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(3, 0b101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b101, 3, Repeat::Never);
            ledout.update()?;
            ledout.update()?;
            ledout.update()?;
            Ok(())
        }
    }

    mod update_onetime_output {
        use super::*;

        #[test]
        fn constant_pattern_one() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(1, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 1, Repeat::Never);
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn constant_pattern_two() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(2, 0b00_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b00, 2, Repeat::Never);
            assert!(!ledout.update()?);
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn symmetric_pattern_one() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(2, 0b10_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b10, 2, Repeat::Never);
            assert!(!ledout.update()?);
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn symmetric_pattern_two() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(4, 0b1001_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1001, 4, Repeat::Never);
            for _ in 1..4 {
                assert!(!ledout.update()?);
            }
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn some_complex_pattern() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(12, 0b1111_0011_1001_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1111_0011_1001_u128, 12, Repeat::Never);
            for _ in 1..12 {
                assert!(!ledout.update()?);
            }
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }
    }

    mod update_repeats {
        use super::*;

        #[test]
        fn never() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(2, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 2, Repeat::Never);
            assert!(!ledout.update()?);
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn zero_times() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(2, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 2, Repeat::Times(0));
            assert!(!ledout.update()?);
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn one_time() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(4, 0b101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 2, Repeat::Times(1));
            for _ in 1..4 {
                assert!(!ledout.update()?);
            }
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn five_times() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(12, 0b_0101_0101_0101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 2, Repeat::Times(5));
            for _ in 1..12 {
                assert!(!ledout.update()?);
            }
            assert!(ledout.update()?);
            assert!(ledout.update()?);
            Ok(())
        }

        #[test]
        fn forever() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(6, 0b01_0101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b1, 2, Repeat::Forever);
            for _ in 1..=6 {
                assert!(!ledout.update()?);
            }
            Ok(())
        }
    }
}

mod fn_position_of_highest_one {
    use super::super::position_of_highest_one;

    #[test]
    fn all_zeros() {
        assert_eq!(0, position_of_highest_one(0));
    }

    #[test]
    fn one_at_pos_zero() {
        assert_eq!(0, position_of_highest_one(0b1));
    }

    #[test]
    fn one_at_pos_one() {
        assert_eq!(1, position_of_highest_one(0b10));
    }

    #[test]
    fn one_at_various_positions() {
        assert_eq!(8, position_of_highest_one(0x1ff));
        assert_eq!(2, position_of_highest_one(0b101));
        assert_eq!(3, position_of_highest_one(0b1010));
    }

    #[test]
    fn all_one() {
        assert_eq!(127, position_of_highest_one(!0));
    }
}
