//! DMAMUX channels.

#[cfg(any(stm32_mcu = "stm32h743", stm32_mcu = "stm32h753",))]
mod h7;
#[cfg(any(
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
mod l4_plus;

#[cfg(any(stm32_mcu = "stm32h743", stm32_mcu = "stm32h753",))]
pub use self::h7::*;
#[cfg(any(
    stm32_mcu = "stm32l4r5",
    stm32_mcu = "stm32l4r7",
    stm32_mcu = "stm32l4r9",
    stm32_mcu = "stm32l4s5",
    stm32_mcu = "stm32l4s7",
    stm32_mcu = "stm32l4s9"
))]
pub use self::l4_plus::*;
