//! Output of a sequence of on/off states
//!
//! * the state sequence can be modified
//! * the sequence can be repeated
//! * An `update()` should be called periodically progress the output
//!   in time.
//!
//! # Implementation
//!
//! * The OutputLED Structure aggregates a GPIO, (bound at init)
//! * Output Pattern can be changed, while the GPIO binding is fixed
//! * A trait (for update) is used to cope with propagate GPIO errors
//! * A prelude simplifies getting the trait in scope
//!
//! # Example
//!
//! For examples check the examples directory
//! directory in the repository.
//!
//! ```rust,ignore
//! use on_off_sequence_output::prelude::*;
//!
//! // This is up to the implementation details of the embedded_hal you are using.
//! let led_pin: OutputPin = hal_function_which_returns_output_pin();
//!
//! const UPDATE_SCALE: u16 = 500;
//! let mut led = OnOffSequenceOutput::new(led_pin, UPDATE_SCALE);
//!
//! let output_states = 0b10011101;
//! let number_of_output_states = 8;
//! led.set(output_states, number_of_output_states, Repeat::Never)
//! loop {
//!    if led.update().unwrap() { break; };
//!    wait(1.ms());
//! }
//!
//! led.set(0b010, 3, Repeat::Times(2))
//! loop {
//!    if led.update().unwrap() { break; };
//!    wait(1.ms());
//! }
//!
//! led.set(0b10, 2, Repeat::Forever)
//! loop {
//!    led.update().unwrap();
//!    wait(1.ms());
//! }

//! ```

#![no_std]

pub mod prelude;

pub mod morse;

use embedded_hal::digital::v2::OutputPin;
use morse::{str_to_morse, MorseError};
// use bitset_core::BitSet;

/// How often shall the output repeated
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Repeat {
    Never,
    Times(u16),
    Forever,
}

/// OutputUpdate Trait which provides an `update()` method
pub trait OutputUpdate {
    type Error;

    /// Updates the output logic and potentially switches the LED state
    ///
    /// # Returns
    ///
    /// * Error - if the hardware GPIO switch to on/off failed
    /// * true - if no further update repetitions are necessary to complete the
    ///   the output
    /// * false - otherwise
    ///
    /// # Notes
    ///
    /// * Needs to be called periodically.
    /// * Side Effect: calls the aggregated GPIO pin to switch on/off
    fn update(&mut self) -> Result<bool, Self::Error>;
}

/// Output of blinking patterns on an LED
pub struct OnOffSequenceOutput<T: OutputPin> {
    /// The wrapped output pin.
    pub pin: T,

    /// The update scaler: the clock rate at wich the output state changes
    /// is equivalent the frequency of the update calls times *update_scale*
    update_scale: u16,

    /// The repeat configuration
    repeat: Repeat,

    /// The output states are represented by the bits of an unsigned 128-bit integer
    output_states: u128,

    /// How many bits are considered (min 1, max: 128)
    number_of_output_states: u16,

    /// Internal state: Manage scaling
    scale_index: u16,

    /// internal state: Manage next output state
    state_index: u16,

    /// Internal state: Run output indicator
    ///
    /// # Values
    ///
    /// * true - either a run is not completed or there are more repetitions to do
    /// * false - run is completed (intermediate) and no more repetitions are
    ///   needed.
    run_output: bool,
}

impl<T: OutputPin> OnOffSequenceOutput<T> {
    /// Initializes a new led output
    ///
    /// # Arguments
    ///
    /// * `pin` - An as output initialized GPIO pin
    /// * `update_scale` - Scale factor:
    ///      state change frequency = update frequency * update_scale
    ///
    /// # Notes
    ///
    /// * Default is symmetrically blinking forever
    pub fn new(pin: T, update_scale: u16) -> Self {
        Self {
            pin,
            update_scale,
            output_states: 0b_10_u128,
            number_of_output_states: 2,
            repeat: Repeat::Forever,
            scale_index: 0u16,
            state_index: 0u16,
            run_output: true,
        }
    }

    fn reinitialize_internal_state(&mut self) {
        self.scale_index = 0u16;
        self.state_index = 0u16;
        self.run_output = true;
    }

    /// Set a new output
    ///
    /// # Arguments
    ///
    /// * `output_states` - bits of a unsigned number: 1 equals on; 0 equals off
    ///   The bits are processes from lsb to msb.
    /// * `number_of_output_states` - how many bits of the fixed number are
    ///   considered to for the output state sequence counted from lsb
    /// * `repeat` - How often is the pattern repeated
    ///
    pub fn set(&mut self, output_states: u128, number_of_output_states: u16, repeat: Repeat) {
        if number_of_output_states > 127 {
            panic!("Must be less than 128 output states");
        };
        if number_of_output_states == 0 {
            panic!("Zero output states do not make sense");
        };
        self.output_states = output_states;
        self.number_of_output_states = number_of_output_states;
        self.repeat = repeat;
        self.reinitialize_internal_state();
    }

    /// Set a new morse code as output
    ///
    /// # Arguments
    ///
    /// * `morse_text` - Short text to be output as morse code sequence
    /// * `repeat` - How often the morse text is repeated
    ///
    /// # Returns
    ///
    /// A result structure
    ///
    /// * with empty value if Ok()
    /// * or Err(MorseError)
    pub fn set_morse(&mut self, morse_text: &str, repeat: Repeat) -> Result<(), MorseError> {
        let t = str_to_morse(morse_text)?;
        self.output_states = t.0;
        self.number_of_output_states = t.1;
        self.reinitialize_internal_state();
        self.repeat = repeat;
        Ok(())
    }
}

/// check if a certain position is set
fn state_at_position(states: u128, position: u16) -> bool {
    let mask: u128 = 1 << position;
    if (states & mask) == 0 {
        return false;
    }
    true
}

impl<T: OutputPin> OutputUpdate for OnOffSequenceOutput<T> {
    type Error = T::Error;

    /// Updates the output logic and potentially switches the LED state
    fn update(&mut self) -> Result<bool, Self::Error> {
        // handle the update scale
        self.scale_index += 1;
        if self.update_scale > self.scale_index {
            return Ok(!self.run_output);
        }
        self.scale_index = 0;

        // handle the output sequence
        if self.run_output {
            // if we get here, always some output has to happen
            if state_at_position(self.output_states, self.state_index) {
                self.pin.set_high()?;
            } else {
                self.pin.set_low()?;
            }
            self.state_index += 1;
            if self.state_index >= self.number_of_output_states {
                // all states are "printed"
                self.run_output = false;
                self.state_index = 0;
            } else {
            }
        }

        // handle the repetitions
        if !self.run_output {
            self.repeat = match self.repeat {
                Repeat::Never => Repeat::Never,
                Repeat::Forever => Repeat::Forever,
                Repeat::Times(n) => {
                    if n > 0 {
                        Repeat::Times(n - 1)
                    } else {
                        Repeat::Never
                    }
                }
            };
            self.run_output = match self.repeat {
                Repeat::Never => false,
                Repeat::Forever => true,
                Repeat::Times(_) => true,
            };
        }

        Ok(!self.run_output)
    }
}

/// Determine at position of the most left one in the bitfield represented as u128
///
/// # Arguments
///
/// * `bitfield` -  The bitfield to find the most left bit that is set to one
///
/// # Returns
///
/// *
pub fn position_of_highest_one(bitfield: u128) -> u16 {
    const MSB_ONE: u128 = 1 << 127;

    let mut position = 127_u16;
    let mut bitfield = bitfield;
    while (bitfield & MSB_ONE) == 0 && position > 0 {
        bitfield = bitfield << 1;
        position -= 1;
    }
    position
}

pub mod macros {
    /// Simplified setting of the output without repetitions
    ///
    /// The number of the output states is automatically computed
    /// It requires that the last output state equals to one
    ///
    /// # Arguments
    ///
    /// * `Instance of OnOffSequenceOutput`
    /// * `bitfield (u128)` - MSB of the Output sequence must be one
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// set_output_once!(ledout, 0b1100);
    /// // ... is equivalent to ...
    /// // ledout.set(0b1100, 4, Repeat::Never);
    /// ```
    #[macro_export]
    macro_rules! set_output_once {
        ($a:expr, $b:expr) => {
            $a.set($b, position_of_highest_one($b), Repeat::Never)
        };
    }

    /// Simplified setting of the output with infinite repetitions
    ///
    /// The number of the output states is automatically computed
    /// It requires that the last output state equals to one
    ///
    /// # Arguments
    ///
    /// * `Instance of OnOffSequenceOutput`
    /// * `bitfield (u128)` - MSB of the Output sequence must be one
    ///
    /// # Examples
    /// ```rust,ignore
    /// set_output_forever!(ledout, 0b1000);
    /// // ... is equivalent to ...
    /// // ledout.set(0b1000, 4, Repeat::Forever);
    /// ```

    #[macro_export]
    macro_rules! set_output_forever {
        ($a:expr, $b:expr) => {
            $a.set($b, position_of_highest_one($b), Repeat::Forever)
        };
    }
}

#[cfg(test)]
mod tests;
