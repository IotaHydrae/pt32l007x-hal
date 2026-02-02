#![no_main]
#![no_std]

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use pt32l007x_hal::{self as _, delay::Delay, pins};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello from PT32L007!");

    let _ = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::default();

    let mut led = pins::PD2::into_push_pull_output();
    let mut led2 = pins::PD3::into_push_pull_output();

    loop {
        led.set_high();
        led2.set_high();
        delay.delay_ms(500);

        led.set_low();
        led2.set_low();
        delay.delay_ms(500);
    }
}
