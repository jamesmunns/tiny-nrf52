#![no_std]
#![no_main]

// Panic provider crate
use panic_reset as _;


// Used to set the program entry point
use cortex_m_rt::entry;

// Provides definitions for our development board
use dwm1001::{
    nrf52832_hal::{
        prelude::*,
    },
    DWM1001,
};

#[entry]
fn main() -> ! {
    let mut board = DWM1001::take().unwrap();
    let mut timer = board.TIMER0.constrain();
    let mut _rng = board.RNG.constrain();
    let mut toggle = false;

    loop {
        // board.leds.D9  - Top LED GREEN
        // board.leds.D12 - Top LED RED
        // board.leds.D11 - Bottom LED RED
        // board.leds.D10 - Bottom LED BLUE
        if toggle {
            board.leds.D10.enable();
        } else {
            board.leds.D10.disable();
        }

        toggle = !toggle;

        // nRF52 requires data to be in RAM, not Flash
        const MSG: &[u8] = "Hello, world!\r\n".as_bytes();
        let mut buf = [0u8; MSG.len()];
        buf.copy_from_slice(MSG);

        board.uart.write(&buf).unwrap();

        timer.delay(1_000_000);
    }
}
