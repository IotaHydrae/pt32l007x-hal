use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin, PinState};
use paste::paste;
use pt32l007x_pac::{GPIOA, GPIOB, GPIOC, GPIOD};

pub mod gpio {
    pub struct Input;
    pub struct Output;
    pub struct OpenDrain;
}

macro_rules! define_pins {
    ($($PinName:ident: ($Peripheral:ident, $N:expr)), *) => {
        $(
            paste! {
                pub struct $PinName<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl $PinName<gpio::Output> {
                    pub fn into_push_pull_output() -> Self {
                        /* Port x pin y output enable */
                        $Peripheral.oes().modify(|f| f.[<set_io $N>](true));

                        /* Port x pin y output open-drain disable */
                        $Peripheral.odc().write(|w| w.[<set_io $N>](true));

                        /* The bit on GPIOx_DR will enable its P-MOS (push-pull) */
                        $Peripheral.dr().modify(|f| f.[<set_dr $N>](true));
                        Self { _mode: PhantomData }
                    }

                    pub fn into_open_drain_output() -> Self {
                        $Peripheral.oes().modify(|f| f.[<set_io $N>](true));

                        /* Port x pin y output open-drain enable */
                        $Peripheral.ods().write(|w| w.[<set_io $N>](true));

                        /* The bit on the (GPIOx_DR register) puts the port in
                         * a high-impedance state (PMOS is never turned on).
                         */
                        $Peripheral.dr().modify(|f| f.[<set_dr $N>](true));

                        Self { _mode: PhantomData }
                    }
                }

                impl ErrorType for $PinName<gpio::Output> {
                    type Error = Infallible;
                }

                impl OutputPin for $PinName<gpio::Output> {
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        $Peripheral.brr().write(|w| w.[<set_io $N>](true));
                        Ok(())
                    }

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        $Peripheral.bsr().write(|w| w.[<set_io $N>](true));
                        Ok(())
                    }

                    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
                        match state {
                            PinState::Low => self.set_low(),
                            PinState::High => self.set_high(),
                        }
                    }
                }
            } /* paste! */
        )*
    };
}

define_pins! {
    PA1: (GPIOA, 1),
    PA2: (GPIOA, 2),
    PA3: (GPIOA, 3),

    PB1: (GPIOB, 1),
    PB4: (GPIOB, 4),
    PB5: (GPIOB, 5),

    PC1: (GPIOC, 1),
    PC2: (GPIOC, 2),
    PC3: (GPIOC, 3),
    PC4: (GPIOC, 4),
    PC5: (GPIOC, 5),
    PC6: (GPIOC, 6),

    PD2: (GPIOD, 2),
    PD3: (GPIOD, 3),
    PD4: (GPIOD, 4),
    PD5: (GPIOD, 5),
    PD6: (GPIOD, 6)
}
