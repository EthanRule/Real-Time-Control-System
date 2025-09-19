#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    pac,
    watchdog::Watchdog,
};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Get the peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    
    // Set up the watchdog driver
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    
    // Configure clocks (required for proper operation)
    let _clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Print hello world
    hprintln!("Hello, world!");
    
    // Keep the program running
    loop {}
}
