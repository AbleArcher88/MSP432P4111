#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::asm;

use msp432p4111;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // we start by initialising peripherals. Don't worry about concurrency, that comes later (in a future example).
    let periph = msp432p4111::Peripherals::take().unwrap(); // take our peripherals
    let wdt = periph.WDT_A; // get a watchdog
    let pgpio = periph.DIO; // get the GPIO ports

    wdt.wdtctl.write(|w| { // initialise the watchdog? Apparently you need a password.
        unsafe {
            w.wdtpw().bits(0x5A);
        }
        w.wdthold().bit(true)
    });

    pgpio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() | 1) }); // set P1_0 to an output bit

    loop {
        pgpio.paout.modify(|r,w| unsafe { w.p1out().bits(r.p1out().bits() | 1) }); // set P1_0 high
        asm::delay(1000000); // a lame way to spin the core

        pgpio.paout.modify(|r,w| unsafe { w.p1out().bits(r.p1out().bits() & 0) }); // set P1_0 low
        asm::delay(1000000); // a lame way to spin the core
    }
}
