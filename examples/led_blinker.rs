#![no_main]
#![no_std]

use cortex_m_rt::entry;
use msp432p4111;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let periph = msp432p4111::Peripherals::take().unwrap();

    let dio = periph.DIO;
    loop {}
}
