#![no_main]
#![no_std]

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, StatefulOutputPin},
};
use pt32l007x_hal::{self as _, delay::Delay, pins};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello from PT32L007!");

    let _ = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::default();

    let mut led = pins::PD2::into_push_pull_output();
    let mut button = pins::PC3::into_pull_up_input();
    let mut count: u32 = 0;

    loop {
        if button.is_low().unwrap() {
            defmt::println!("button pressed! {}", count);
            count = count.saturating_add(1);

            led.toggle();
        }

        delay.delay_ms(180);
    }
}
