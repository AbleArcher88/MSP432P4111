#![no_main]
#![no_std]

use cortex_m_rt::entry;
use msp432p4111;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let periph = msp432p4111::Peripherals::take().unwrap();
    let wdt = periph.WDT_A;
    let pgpio = periph.DIO;

    wdt.wdtctl.write(|w| {
        unsafe {
            w.wdtpw().bits(0x5A);
        }
        w.wdthold().bit(true);
    });



    loop {}
}
