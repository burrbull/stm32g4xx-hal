//! Comparator
//!
//! ## Origin
//!
//! This code has been taken from the stm32g0xx-hal project and modified slightly to support
//! STM32G4xx MCUs.

use core::marker::PhantomData;

use crate::dac;
use crate::exti::{Event as ExtiEvent, ExtiExt};
use crate::gpio::{self, alt::CompOutput, Analog, SignalEdge};

use crate::rcc::{Clocks, Rcc};
//use crate::stasis;
use crate::stm32::{COMP, EXTI};

/// Enabled Comparator (type state)
pub struct Enabled;

/// Enabled and locked (config is read only)
pub struct Locked;

/// Disabled Comparator (type state)
pub struct Disabled;

pub trait ED {}
impl ED for Enabled {}
impl ED for Disabled {}

pub trait EnabledState {}
impl EnabledState for Enabled {}
impl EnabledState for Locked {}

// TODO: Split COMP in PAC

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config {
    //power_mode: PowerMode,
    hysteresis: Hysteresis,
    inverted: bool,
    //output_xor: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hysteresis: Hysteresis::None,
            inverted: false,
            //power_mode: PowerMode::HighSpeed,
            //output_xor: false,
        }
    }
}

impl Config {
    pub fn hysteresis(mut self, hysteresis: Hysteresis) -> Self {
        self.hysteresis = hysteresis;
        self
    }

    pub fn output_inverted(mut self) -> Self {
        self.inverted = true;
        self
    }

    pub fn output_polarity(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Hysteresis {
    None = 0b000,
    H10mV = 0b001,
    H20mV = 0b010,
    H30mV = 0b011,
    H40mV = 0b100,
    H50mV = 0b101,
    H60mV = 0b110,
    H70mV = 0b111,
}

pub trait Comp: CompOutput {
    fn csr(&self) -> &crate::pac::comp::CCSR;
    type InP: InputPlus;
    type InM: InputMinus;
    const EVENT: ExtiEvent;
}

pub trait InputPlus {
    fn inpsel(&self) -> bool;
}

pub trait InputMinus {
    fn inmsel(&self) -> u8;
    fn use_vrefint(&self) -> bool;
    fn use_resistor_divider(&self) -> bool;
}

macro_rules! input {
    (
        $COMP:ident,
        $n:literal,
        $INP1:ident, $INP2:ident,
        $DC1:ident, $DC2:ident,
        $INM1:ident, $INM2:ident
    ) => {
        pub struct $COMP {
            pub(super) _rb: PhantomData<()>,
        }

        impl Comp for $COMP {
            fn csr(&self) -> &$crate::pac::comp::CCSR {
                // SAFETY: The COMP1 type is only constructed with logical ownership of
                // these registers.
                &unsafe { &*COMP::ptr() }.ccsr($n)
            }
            type InP = InP;
            type InM = InM;
            const EVENT: ExtiEvent = ExtiEvent::$COMP;
        }

        #[derive(Debug)]
        pub enum InP {
            $INP1(gpio::$INP1<Analog>),
            $INP2(gpio::$INP2<Analog>),
        }

        impl InputPlus for InP {
            fn inpsel(&self) -> bool {
                match self {
                    Self::$INP1(_) => false,
                    Self::$INP2(_) => true,
                }
            }
        }

        impl From<gpio::$INP1<Analog>> for InP {
            fn from(pin: gpio::$INP1<Analog>) -> Self {
                Self::$INP1(pin)
            }
        }

        impl From<gpio::$INP2<Analog>> for InP {
            fn from(pin: gpio::$INP2<Analog>) -> Self {
                Self::$INP2(pin)
            }
        }

        paste::paste! {
            pub enum InM {
                VRefintM14,
                VRefintM12,
                VRefintM34,
                VRefint,
                [<$DC1 Mix>](dac::$DC1<{ dac::M_MIX_SIG }, dac::Enabled>),
                [<$DC1 Int>](dac::$DC1<{ dac::M_INT_SIG }, dac::Enabled>),
                [<$DC2 Mix>](dac::$DC2<{ dac::M_MIX_SIG }, dac::Enabled>),
                [<$DC2 Int>](dac::$DC2<{ dac::M_INT_SIG }, dac::Enabled>),
                $INM1(gpio::$INM1<Analog>),
                $INM2(gpio::$INM2<Analog>),
            }

            impl InputMinus for InM {
                fn inmsel(&self) -> u8 {
                    match self {
                        Self::VRefintM14 => 0,
                        Self::VRefintM12 => 1,
                        Self::VRefintM34 => 2,
                        Self::VRefint => 3,
                        Self::[<$DC1 Mix>](_) | InM::[<$DC1 Int>](_) => 4,
                        Self::[<$DC2 Mix>](_) | InM::[<$DC2 Int>](_) => 5,
                        Self::$INM1(_) => 6,
                        Self::$INM2(_) => 7,
                    }
                }
                fn use_vrefint(&self) -> bool {
                    matches!(self, Self::VRefintM14 | Self::VRefintM12 | Self::VRefintM34 | Self::VRefint)
                }
                fn use_resistor_divider(&self) -> bool {
                    matches!(self, Self::VRefintM14 | Self::VRefintM12 | Self::VRefintM34)
                }
            }

            impl From<dac::$DC1<{ dac::M_MIX_SIG }, dac::Enabled>> for InM {
                fn from(token: dac::$DC1<{ dac::M_MIX_SIG }, dac::Enabled>) -> Self {
                    Self::[<$DC1 Mix>](token)
                }
            }

            impl From<dac::$DC1<{ dac::M_INT_SIG }, dac::Enabled>> for InM {
                fn from(token: dac::$DC1<{ dac::M_INT_SIG }, dac::Enabled>) -> Self {
                    Self::[<$DC1 Int>](token)
                }
            }

            impl From<dac::$DC2<{ dac::M_MIX_SIG }, dac::Enabled>> for InM {
                fn from(token: dac::$DC2<{ dac::M_MIX_SIG }, dac::Enabled>) -> Self {
                    Self::[<$DC2 Mix>](token)
                }
            }

            impl From<dac::$DC2<{ dac::M_INT_SIG }, dac::Enabled>> for InM {
                fn from(token: dac::$DC2<{ dac::M_INT_SIG }, dac::Enabled>) -> Self {
                    Self::[<$DC2 Int>](token)
                }
            }
        }

        impl From<gpio::$INM1<Analog>> for InM {
            fn from(pin: gpio::$INM1<Analog>) -> Self {
                Self::$INM1(pin)
            }
        }

        impl From<gpio::$INM2<Analog>> for InM {
            fn from(pin: gpio::$INM2<Analog>) -> Self {
                Self::$INM2(pin)
            }
        }
    };
}
use input;

pub mod comp1 {
    use super::*;
    input!(COMP1, 0, PA1, PB1, Dac3Ch1, Dac1Ch1, PA4, PA0);
}
pub use comp1::COMP1;

pub mod comp2 {
    use super::*;
    input!(COMP2, 1, PA7, PA3, Dac3Ch2, Dac1Ch2, PA5, PA2);
}
pub use comp2::COMP2;

pub mod comp3 {
    use super::*;
    input!(COMP3, 2, PA0, PC1, Dac3Ch1, Dac1Ch1, PF1, PC0);
}
pub use comp3::COMP3;

pub mod comp4 {
    use super::*;
    input!(COMP4, 3, PB0, PE7, Dac3Ch2, Dac1Ch1, PE8, PB2);
}
pub use comp4::COMP4;

#[cfg(feature = "comp5")]
pub mod comp5 {
    use super::*;
    input!(COMP5, 4, PB13, PD12, Dac4Ch1, Dac1Ch2, PB10, PD13);
}
#[cfg(feature = "comp5")]
pub use comp5::COMP5;

#[cfg(feature = "comp6")]
pub mod comp6 {
    use super::*;
    input!(COMP6, 5, PB11, PD11, Dac4Ch2, Dac2Ch1, PD10, PB15);
}
#[cfg(feature = "comp5")]
pub use comp6::COMP6;

#[cfg(feature = "comp7")]
pub mod comp7 {
    use super::*;
    input!(COMP7, 6, PB14, PD14, Dac4Ch1, Dac2Ch1, PD15, PB12);
}
#[cfg(feature = "comp5")]
pub use comp7::COMP7;

pub struct Comparator<COMP: Comp, ED> {
    regs: COMP,
    inp: COMP::InP,
    inm: COMP::InM,
    _enabled: PhantomData<ED>,
}

pub trait ComparatorExt: Sized + Comp {
    /// Initializes a comparator
    fn comparator(
        self,
        positive_input: impl Into<Self::InP>,
        negative_input: impl Into<Self::InM>,
        config: Config,
        clocks: &Clocks,
    ) -> Comparator<Self, Disabled>;
}

impl<COMP: Comp> ComparatorExt for COMP {
    fn comparator(
        self,
        positive_input: impl Into<Self::InP>,
        negative_input: impl Into<Self::InM>,
        config: Config,
        clocks: &Clocks,
    ) -> Comparator<COMP, Disabled> {
        let positive_input = positive_input.into();
        let negative_input = negative_input.into();
        // Delay for scaler voltage bridge initialization for certain negative inputs
        let voltage_scaler_delay = clocks.sys_clk.raw() / (1_000_000 / 200); // 200us
        cortex_m::asm::delay(voltage_scaler_delay);
        self.csr().modify(|_, w| unsafe {
            w.hyst().bits(config.hysteresis as u8);
            w.scalen().bit(negative_input.use_vrefint());
            w.brgen().bit(negative_input.use_resistor_divider());
            w.pol().bit(config.inverted)
        });

        Comparator {
            regs: self,
            inp: positive_input,
            inm: negative_input,
            _enabled: PhantomData,
        }
    }
}

impl<COMP: Comp> Comparator<COMP, Disabled> {
    /// Initializes a comparator
    pub fn new(
        comp: COMP,
        positive_input: impl Into<COMP::InP>,
        negative_input: impl Into<COMP::InM>,
        config: Config,
        clocks: &Clocks,
    ) -> Self {
        comp.comparator(positive_input, negative_input, config, clocks)
    }

    /// Enables the comparator
    pub fn enable(self) -> Comparator<COMP, Enabled> {
        self.regs.csr().modify(|_, w| w.en().set_bit());
        Comparator {
            regs: self.regs,
            inp: self.inp,
            inm: self.inm,
            _enabled: PhantomData,
        }
    }

    /// Enables raising the `ADC_COMP` interrupt at the specified output signal edge
    pub fn listen(&self, edge: SignalEdge, exti: &EXTI) {
        exti.listen(COMP::EVENT, edge);
    }
}

impl<COMP: Comp, ED: EnabledState> Comparator<COMP, ED> {
    /// Returns the value of the output of the comparator
    pub fn output(&self) -> bool {
        self.regs.csr().read().value().bit_is_set()
    }
}

impl<COMP: Comp> Comparator<COMP, Enabled> {
    pub fn lock(self) -> Comparator<COMP, Locked> {
        // Setting this bit turns all other bits into read only until restart
        self.regs.csr().modify(|_, w| w.lock().set_bit());
        Comparator {
            regs: self.regs,
            inp: self.inp,
            inm: self.inm,
            _enabled: PhantomData,
        }
    }

    /// Disables the comparator
    pub fn disable(self) -> Comparator<COMP, Disabled> {
        self.regs.csr().modify(|_, w| w.en().clear_bit());
        Comparator {
            regs: self.regs,
            inp: self.inp,
            inm: self.inm,
            _enabled: PhantomData,
        }
    }
}

impl<COMP: Comp, ED> Comparator<COMP, ED> {
    /// Disables raising interrupts for the output signal
    pub fn unlisten(&self, exti: &EXTI) {
        exti.unlisten(COMP::EVENT);
    }

    /// Returns `true` if the output signal interrupt is pending for the `edge`
    pub fn is_pending(&self, exti: &EXTI) -> bool {
        exti.is_pending(COMP::EVENT)
    }

    /// Unpends the output signal interrupt
    pub fn unpend(&self, exti: &EXTI) {
        exti.unpend(COMP::EVENT);
    }
}

impl<COMP: Comp, ED> Comparator<COMP, ED> {
    /// Configures a GPIO pin to output the signal of the comparator
    ///
    /// Multiple GPIO pins may be configured as the output simultaneously.
    pub fn output_pin<Otype>(&self, pin: impl Into<COMP::Out<Otype>>) {
        let _pin = pin.into();
    }
}

#[cfg(not(feature = "comp7"))]
type Comparators = (COMP1, COMP2, COMP3, COMP4);

#[cfg(feature = "comp7")]
type Comparators = (COMP1, COMP2, COMP3, COMP4, COMP5, COMP6, COMP7);

/// Enables the comparator peripheral, and splits the [`COMP`] into independent [`COMP1`] and [`COMP2`]
pub fn split(_comp: COMP, rcc: &mut Rcc) -> Comparators {
    // Enable COMP, SYSCFG, VREFBUF clocks
    rcc.rb.apb2enr().modify(|_, w| w.syscfgen().set_bit());

    // Reset COMP, SYSCFG, VREFBUF
    rcc.rb.apb2rstr().modify(|_, w| w.syscfgrst().set_bit());
    rcc.rb.apb2rstr().modify(|_, w| w.syscfgrst().clear_bit());

    (
        COMP1 { _rb: PhantomData },
        COMP2 { _rb: PhantomData },
        COMP3 { _rb: PhantomData },
        COMP4 { _rb: PhantomData },
        #[cfg(feature = "comp5")]
        COMP5 { _rb: PhantomData },
        #[cfg(feature = "comp6")]
        COMP6 { _rb: PhantomData },
        #[cfg(feature = "comp7")]
        COMP7 { _rb: PhantomData },
    )
}

pub trait ComparatorSplit {
    /// Enables the comparator peripheral, and splits the [`COMP`] into independent [`COMP1`] and [`COMP2`]
    fn split(self, rcc: &mut Rcc) -> Comparators;
}

impl ComparatorSplit for COMP {
    fn split(self, rcc: &mut Rcc) -> Comparators {
        split(self, rcc)
    }
}
