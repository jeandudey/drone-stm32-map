//! Quad Serial Peripheral Interface.

#[cfg(any(stm32_mcu = "stm32h743", stm32_mcu = "stm32h753",))]
mod h7;

#[cfg(any(stm32_mcu = "stm32h743", stm32_mcu = "stm32h753",))]
pub use self::h7::*;
