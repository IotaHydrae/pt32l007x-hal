use cortex_m::asm;
use embedded_hal::delay::DelayNs;
use fugit::HertzU32;

pub struct Delay {
    sysclk_freq: u32,
}

impl Delay {
    pub fn new(sysclk: HertzU32) -> Self {
        Self {
            sysclk_freq: sysclk.to_Hz(),
        }
    }

    #[inline]
    fn busy_wait_cycles(&self, cycles: u32) {
        let mut remain = cycles;
        while remain > 0 {
            asm::nop();
            remain = remain.saturating_sub(1);
        }
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new(HertzU32::kHz(2500))
    }
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        let cycles = (ns as u64 * self.sysclk_freq as u64 / 1_000_000_000) as u32;
        self.busy_wait_cycles(cycles);
    }

    fn delay_us(&mut self, us: u32) {
        let cycles = (us as u64 * self.sysclk_freq as u64 / 1_000_000) as u32;
        self.busy_wait_cycles(cycles);
    }

    fn delay_ms(&mut self, ms: u32) {
        for _ in 0..ms {
            self.delay_us(1000);
        }
    }
}
