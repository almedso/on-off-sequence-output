# `on-off-sequence-output`

[![Build Status](https://travis-ci.org/almedso/on-off-sequence-output.svg)](https://travis-ci.org/almedso/on-off-sequence-output)
[![Crate](https://img.shields.io/crates/v/on-off-sequence-output.svg)](https://crates.io/crates/on-off-sequence-output)
[![Docs](https://docs.rs/on-off-sequence-output/badge.svg)](https://docs.rs/debounced-pin)

A platform-agnostic library that provides output of a sequence of on/off states
e.g. on an LED.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
on-off-sequence-output = "0.1"
```

## Usage

This is a library crate and should work with any led that
implements 'embedded_hal' trait 'OutputPin' on any cortex-m MCU.

The source could look like ...

```rust
use on_off_sequence_output::prelude::*;

// This is up to the implementation details of the embedded_hal you are using.
let led_pin: OutputPin = hal_function_which_returns_output_pin();

const UPDATE_SCALE: u16 = 500;
let mut led = OnOffSequenceOutput::new(led_pin, UPDATE_SCALE);

let output_states = 0b10011101;
let number_of_output_states = 8;
led.set(output_states, number_of_output_states, Repeat::Never)
loop {
   if led.update().unwrap() { break; };
   wait(1.ms());
}

led.set(0b010, 3, Repeat::Times(2))
loop {
   if led.update().unwrap() { break; };
   wait(1.ms());
}

led.set(0b10, 2, Repeat::Forever)
loop {
   led.update().unwrap();
   wait(1.ms());
}
```

The `examples` folder contains a working example named `show-led-output`.
The cargo tooling (.cargo, memory.x, openocd.cfg ..) is prepared for an STM NUCLEO F401RE evaluation board.

If you have that board avaliable run

```sh
cargo build --target thumbv7em-none-eabihf --example show-led-output
openocd-flash.sh target/thumbv7em-none-eabihf/release/examples/show-led-output
```

Since this is a library, there is no toolchain configured for build in `.cargo/config`.

## Testing

Testing is done via unit tests on host only. Run

```sh
cargo test --lib --tests
```

... to exclude examples because they do not compile on host

## License

This project is licensed under

- MIT License ([`LICENSE.md`](LICENSE.md) or
  [online](https://opensource.org/licenses/MIT))

[`embedded-hal`]: https://docs.rs/crate/embedded-hal
