use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin, PinState};
use pt32l007x_pac::GPIOD;

pub mod gpio {
    pub struct Input;
    pub struct Output;
    pub struct OpenDrain;
}

pub struct PD2<MODE> {
    _mode: PhantomData<MODE>,
}

impl PD2<gpio::Output> {
    pub fn into_push_pull_output() -> Self {
        GPIOD.oes().modify(|f| f.set_io2(true));

        Self { _mode: PhantomData }
    }
}

impl ErrorType for PD2<gpio::Output> {
    type Error = Infallible;
}

impl OutputPin for PD2<gpio::Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        GPIOD.dr().modify(|f| f.set_dr2(false));
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        GPIOD.dr().modify(|f| f.set_dr2(true));
        Ok(())
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }
}
