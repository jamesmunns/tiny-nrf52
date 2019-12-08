#![no_std]
#![no_main]

// Panic provider crate
use panic_persist;
use cortex_m;

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
    match inner_main() {
        Ok(()) => cortex_m::peripheral::SCB::sys_reset(),
        Err(e) => panic!(e),
    }
}

fn inner_main() -> Result<(), &'static str> {
    let mut board = DWM1001::take().ok_or("Error getting board!")?;
    let mut timer = board.TIMER0.constrain();
    let mut _rng = board.RNG.constrain();
    let mut toggle = false;

    if let Some(msg) = panic_persist::get_panic_message_bytes() {
        // write the error message in reasonable chunks
        for i in msg.chunks(255) {
            board.uart.write(i).map_err(|_| "Error writing error!")?;
        }
    }

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

        board.uart.write(&buf).map_err(|_| "Error writing hello!")?;

        timer.delay(1_000_000);
    }
}
