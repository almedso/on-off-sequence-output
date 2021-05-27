#![no_main]
#![no_std]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use panic_semihosting as _;

use on_off_sequence_output::prelude::*;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();

    // (Re-)configure PA5 (LD2 - User Led) as output
    let pin = gpioa.pa5.into_push_pull_output();

    // Constrain clock registers
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

    // Get delay provider
    let mut delay = Delay::new(cp.SYST, clocks);

    // Setup the LED Output using 5 ticks time upscaling
    // The duration of a state is than 500 ms assuming a length between two
    // updates of 100 ms
    const UPSCALE_FACTOR: u16 = 5;
    let mut ledout = OnOffSequenceOutput::new(pin, UPSCALE_FACTOR);

    // Init done -> Now we can put everything in operation

    // Blink 10 times symmetrically at one Herz
    ledout.set(0b01, 2, Repeat::Times(10));
    loop {
        delay.delay_ms(100_u16);
        if ledout.update().unwrap() {
            break;
        };
    }

    // Switch on two seconds, switch off 1 second, switch on 3 seconds
    ledout.set(0b111111001111, 12, Repeat::Never);
    loop {
        delay.delay_ms(100_u16);
        if ledout.update().unwrap() {
            break;
        };
    }

    // Blink forever at two Herz and duty factor 0.25
    ledout.set(0b1000, 4, Repeat::Times(10));
    loop {
        delay.delay_ms(100_u16);
        ledout.update().unwrap();
    }
}
