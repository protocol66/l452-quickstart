#![no_main]
#![no_std]

// Halt on panic
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32_hal2 as hal;

use crate::hal::{prelude::*,
                 clocks::Clocks,
                 pac,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();
    
    let clocks = Clocks::default();
    clocks.setup(&mut p.RCC, &mut p.FLASH);
    
    loop{

    }
}
