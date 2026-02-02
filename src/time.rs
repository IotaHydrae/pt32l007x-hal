use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};
use cortex_m::peripheral::SYST;
use cortex_m_rt::exception;

static TICK: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

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

pub fn millis() -> u32 {
    interrupt::free(|cs| *TICK.borrow(cs).borrow())
}

pub fn delay_ms(ms: u32) {
    let start = millis();

    while millis().wrapping_sub(start) < ms {
        cortex_m::asm::nop();
    }
}

pub fn tick_init(mut syst: SYST) {
    syst.set_reload(64 * 1000 - 1);
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    syst.enable_counter();
    syst.enable_interrupt();
}
