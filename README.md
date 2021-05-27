# `on-off-sequence-output`

[![Build Status](https://travis-ci.org/almedso/single-led-output.svg)](https://travis-ci.org/almedso/single-led-output)
[![Crate](https://img.shields.io/crates/v/single-led-output.svg)](https://crates.io/crates/single-led-output)
[![Docs](https://docs.rs/single-led-output/badge.svg)](https://docs.rs/debounced-pin)

A platform-agnostic library that provides output of a sequence of on/off states
e.g. on an LED.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
single-led-output = "0.1"
```

This crate currently requires [`embedded-hal`] to be built using the
`unproven` feature, for access to the `OutputPin` trait.

## Usage

This is a library crate and should work with any led that
implements 'embedded_hal' trait 'OutputPin' on any cortex-m MCU.

The code could look like ...

```rust
use single_led_output::prelude::*;

// TODO
```

The `examples` folder contains a working example named `show-led-output`.
The cargo tooling (.cargo, memory.x, openocd.cfg ..) is prepared for an STM NUCLEO F401RE evaluation board.

If you have that board at and and connected run

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
