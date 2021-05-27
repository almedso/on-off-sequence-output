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

    mod update {
        use super::*;

        #[test]
        fn no_update_at_scale_four() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(0, 0b0_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            for _index in 0..2 {
                assert!(!ledout.update()?);
            }
            Ok(())
        }

        #[test]
        fn one_update_at_scale_four() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(1, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            for _index in 0..4 {
                assert!(!ledout.update()?);
            }
            Ok(())
        }

        #[test]
        fn three_updates_at_scale_four() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(3, 0b101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 4);
            ledout.set(0b101, 3, Repeat::Never);
            for _index in 0..12 {
                assert!(!ledout.update()?);
            }
            Ok(())
        }

        #[test]
        fn one_update_at_scale_one() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(1, 0b1_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            assert!(!ledout.update()?);
            Ok(())
        }

        #[test]
        fn three_updates_at_scale_one() -> Result<(), MockedOutputPinError> {
            let pin_mock = MockedOutputPin::expected(3, 0b101_u128);
            let mut ledout = OnOffSequenceOutput::new(pin_mock, 1);
            ledout.set(0b101, 3, Repeat::Never);
            for _index in 0..3 {
                assert!(!ledout.update()?);
            }
            Ok(())
        }
    }
}
