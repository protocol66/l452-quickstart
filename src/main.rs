#![no_main]
#![no_std]

// Halt on panic
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use stm32_hal2 as hal;

use crate::hal::{prelude::*,
                 clocks::Clocks,
                 pac,
};

// Used with semihosting to print to terminal
use core::fmt::Write;

#[entry]
fn main() -> ! {
    // printing to debug terminal
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Initializing....").ok();

    // Grab Peripherals
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Setup Clocks. Configured as 80 MHz sysclk and pll using HSI
    let clocks = Clocks::default();
    clocks.setup(&mut p.RCC, &mut p.FLASH).ok();

    // Rest of setup goes here....

    writeln!(hstdout, "done.").ok();

    loop {

    }
}
