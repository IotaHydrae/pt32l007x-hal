#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};
use cortex_m::peripheral::SYST;
use cortex_m_rt::exception;
use pt32l007x_hal as _; // global logger + panicking-behavior + memory layout
use pt32l007x_pac::{
    self as pac,
    gpioa::regs::{Dr, Oes},
};

static TICK: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello from PT32L007!");

    let p = cortex_m::Peripherals::take().unwrap();
    tick_init(p.SYST);

    /*
     * According to the user manual, if we want to use PD2 as output
     * we need to set bit 2 of `GPIOx_OES(Output Enable Set Register)`
     * and bit 2 of `GPIOx_DR(Data Register)`
     */

    /* set PD2 as output enable */
    pac::GPIOD.oes().write_value(Oes(1 << 2));

    loop {
        /* set PD2 output high */
        pac::GPIOD.dr().write_value(Dr(1 << 2));
        delay_ms(500);

        /* set PD2 output low */
        pac::GPIOD.dr().write_value(Dr(!(1 << 2)));
        delay_ms(500);
    }
}

#[exception]
fn SysTick() {
    tick();
}

#[inline]
pub fn tick() {
    interrupt::free(|cs| {
        *TICK.borrow(cs).borrow_mut() += 1;
    });
}

fn millis() -> u32 {
    interrupt::free(|cs| *TICK.borrow(cs).borrow())
}

fn delay_ms(ms: u32) {
    let start = millis();

    while millis().wrapping_sub(start) < ms {
        cortex_m::asm::nop();
    }
}

fn tick_init(mut syst: SYST) {
    syst.set_reload(60 * 1000);
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    syst.enable_counter();
    syst.enable_interrupt()
}
