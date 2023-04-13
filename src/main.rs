//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use panic_rtt_target as _;
use rtt_target::{rprint, rtt_init_print};

use rp_pico as bsp;

use bsp::hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut peris = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peris.WATCHDOG);
    let _clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        peris.XOSC,
        peris.CLOCKS,
        peris.PLL_SYS,
        peris.PLL_USB,
        &mut peris.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    loop {
        rprint!("aaaaaaaaaaaaaaaaaaaaaa\r\n");
    }
}
