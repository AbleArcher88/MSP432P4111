#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU8, Ordering};
use core::cell::RefCell;
use cortex_m_rt::entry;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;

use msp432p4111 as pac;
use msp432p4111::Interrupt;
use msp432p4111::CorePeripherals;
use panic_halt as _;

const LED_MAX: u8 = 2;

static PERIPHERALS: Mutex<RefCell<Option<msp432p4111::Peripherals>>> = Mutex::new(RefCell::new(None)); // set a static mutex that allows us to safely globalize the peripherals
static LED_STATE: AtomicU8 = AtomicU8::new(0); // Atomic allow us to manipulate statics with safety

#[entry]
fn main() -> ! {

    let cortex_p = cortex_m::Peripherals::take().unwrap();

    let nvic = &cortex_p.NVIC;

    unsafe{
        nvic.iser[1].write(0x08);
    }

    cortex_m::interrupt::free(|cs| {
        PERIPHERALS.borrow(cs).replace(Some(pac::Peripherals::take().unwrap()));

        let periph = PERIPHERALS.borrow(cs).borrow();
        let wdt = periph.as_ref().unwrap().WDT_A;
        let pgpio = periph.as_ref().unwrap().DIO;

        wdt.wdtctl.write(|w| { // initialise the watchdog? Apparently you need a password.
            unsafe {
                w.wdtpw().bits(0x5A);
            }
            w.wdthold().bit(true)
        });

        // use the P1_0 LED to indicate running code
        pgpio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() | 0x01) }); // set P1_0 as output bit

        // configure P1_1 as input and enable the pullup resistor
        pgpio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() & !0x02) }); // configure P1_1 as input
        pgpio.paren.modify(|r, w| unsafe { w.p1ren().bits(r.p1ren().bits() | 0x02) }); // enable pullup resistor
        pgpio.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits() | 0x02) }); // pull up?

        // Enable and configure interrupts
        pgpio.paies.write(|w| unsafe { w.p1ies().bits(0x02) });
        pgpio.paifg.write(|w| unsafe { w.p1ifg().bits(0x00) });
        pgpio.paie.write(|w| unsafe { w.p1ie().bits(0x02) });

        // condfigure and initialise the LED array
        pgpio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 0x07) }); // set P2_0 as output
    });

    loop{
        cortex_m::interrupt::free(|cs| {
            let periph = PERIPHERALS.borrow(cs).borrow();
            let pgpio = &periph.as_ref().unwrap().DIO;
            if LED_STATE.load(Ordering::Relaxed) == 0 {
                pgpio.paout.modify(|r,w| unsafe { w.p2out().bits(r.p2out().bits() | 0x01) }) // Set P2_0 high, all other pins low
            } else if LED_STATE.load(Ordering::Relaxed) == 1 {
                pgpio.paout.modify(|r,w| unsafe { w.p2out().bits(r.p2out().bits() | 0x02) }) // Set P2_1 high, all other pins low
            } else if LED_STATE.load(Ordering::Relaxed) == 2 {
                pgpio.paout.modify(|r,w| unsafe { w.p2out().bits(r.p2out().bits() | 0x03) }) // Set P2_2 high, all other pins low
            }
        });
        asm::wfi();
    }
}

// Make sure to enable "rt" feature at compile time

#[interrupt]
fn PORT1_IRQ() {
    if LED_STATE.load(Ordering::Relaxed) == LED_MAX {
        LED_STATE.store(0, Ordering::Relaxed);
    } else {
        LED_STATE.fetch_add(1, Ordering::Relaxed);
    }
    cortex_m::interrupt::free(|cs| {
        let periph = PERIPHERALS.borrow(cs).borrow();
        let pgpio = &periph.as_ref().unwrap().DIO;
        pgpio.paifg.write(|w| unsafe { w.p1ifg().bits(0x00) });
    });
}
