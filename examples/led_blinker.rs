#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::asm;

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
        w.wdthold().bit(true)
    });

    pgpio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() | 1) });

    loop {
        pgpio.paout.modify(|r,w| unsafe { w.p1out().bits(r.p1out().bits() | 1) });
        asm::delay(1000000);

        pgpio.paout.modify(|r,w| unsafe { w.p1out().bits(r.p1out().bits() & 0) });
        asm::delay(1000000);
    }
}
