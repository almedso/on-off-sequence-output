# `single-led-output`

[![Build Status](https://travis-ci.org/almedso/single-led-output.svg)](https://travis-ci.org/almedso/single-led-output)
[![Crate](https://img.shields.io/crates/v/single-led-output.svg)](https://crates.io/crates/single-led-output)
[![Docs](https://docs.rs/single-led-output/badge.svg)](https://docs.rs/debounced-pin)

A platform-agnostic single led output library.

This library provides an `update()` method to refresh an led according to a preset time pattern.

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

The delivered arbitrary configuration

## License

This project is licensed under

- MIT License ([`LICENSE.md`](LICENSE.md) or
  [online](https://opensource.org/licenses/MIT))

[`embedded-hal`]: https://docs.rs/crate/embedded-hal
