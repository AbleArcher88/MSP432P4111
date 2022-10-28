#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU8, Ordering};
use cortex_m_rt::entry;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;

use msp432p4111 as pac;
use msp432p4111::Interrupt;
use pac::CorePeripherals;
use panic_halt as _;

const LED_MAX: u8 = 2;

static PERIPHERALS: Mutex<msp432p4111::Peripherals> = Mutex::new(msp432p4111::Peripherals::take().unwrap()); // set a static mutex that allows us to safely globalize the peripherals
static LED_STATE: AtomicU8 = AtomicU8::new(0); // Atomic allow us to manipulate statics with safety

#[entry]
fn main() -> ! {

    let periph = Peripherals::take().unwrap();
    let wdt = periph.WDT_A;
    let pgpio = periph.DIO;

    wdt.wdtctl.write(|w| { // initialise the watchdog? Apparently you need a password.
        unsafe {
            w.wdtpw().bits(0x5A);
        }
        w.wdthold().bit(true)
    });


    loop{
        if LED_STATE.load(Ordering::Relaxed) == 0 {
            // Red Light
        } else if LED_STATE.load(Ordering::Relaxed) == 1 {
            // Blue Light
        } else if LED_STATE.load(Ordering::Relaxed) == 2 {
            // Green Light
        }
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
}
