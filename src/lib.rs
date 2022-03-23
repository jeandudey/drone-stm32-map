//! STM32 peripheral mappings for Drone, an Embedded Operating System.
//!
//! This crate uses
//! [CMSIS-SVD](https://arm-software.github.io/CMSIS_5/SVD/html/index.html)
//! files provided by [STMicroelectronics](https://www.st.com/) to automatically
//! generate Drone register and interrupt bindings. However only the
//! corresponding Reference Manual is the single source of truth. A difference
//! between this crate bindings and the Reference Manual is considered a
//! bug. Fixing such a bug is *not a breaking change*.
//!
//! This crate re-exports the contents of [`drone_cortexm::map`] module and is a
//! drop-in replacement for it.
//!
//! # Supported Devices
//!
//! | `stm32_mcu` | Core name             | Reference manual                                                         | Available features                                       |
//! |-------------|-----------------------|--------------------------------------------------------------------------|----------------------------------------------------------|
//! | `stm32f100` | ARM® Cortex®-M3 r1p1  | [RM0041](https://www.st.com/resource/en/reference_manual/cd00246267.pdf) | `dma` `gpio` `spi` `tim`                                 |
//! | `stm32f101` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) | `dma` `gpio` `spi` `tim`                                 |
//! | `stm32f102` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) | `dma` `gpio` `spi` `tim`                                 |
//! | `stm32f103` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) | `dma` `gpio` `spi` `tim`                                 |
//! | `stm32f107` | ARM® Cortex®-M3 r1p1  | [RM0008](https://www.st.com/resource/en/reference_manual/cd00171190.pdf) | `dma` `gpio` `spi` `tim`                                 |
//! | `stm32f303` | ARM® Cortex®-M4F r0p1 | [RM0316](https://www.st.com/resource/en/reference_manual/dm00043574.pdf) | `adc` `dma` `exti` `gpio` `i2c` `tim`                    |
//! | `stm32f401` | ARM® Cortex®-M4F r0p1 | [RM0368](https://www.st.com/resource/en/reference_manual/dm00096844.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f405` | ARM® Cortex®-M4F r0p1 | [RM0090](https://www.st.com/resource/en/reference_manual/dm00031020.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f407` | ARM® Cortex®-M4F r0p1 | [RM0090](https://www.st.com/resource/en/reference_manual/dm00031020.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f410` | ARM® Cortex®-M4F r0p1 | [RM0401](https://www.st.com/resource/en/reference_manual/dm00180366.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f411` | ARM® Cortex®-M4F r0p1 | [RM0383](https://www.st.com/resource/en/reference_manual/dm00119316.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f412` | ARM® Cortex®-M4F r0p1 | [RM0402](https://www.st.com/resource/en/reference_manual/dm00180369.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f413` | ARM® Cortex®-M4F r0p1 | [RM0430](https://www.st.com/resource/en/reference_manual/dm00305666.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f427` | ARM® Cortex®-M4F r0p1 | [RM0090](https://www.st.com/resource/en/reference_manual/dm00031020.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f429` | ARM® Cortex®-M4F r0p1 | [RM0090](https://www.st.com/resource/en/reference_manual/dm00031020.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f446` | ARM® Cortex®-M4F r0p1 | [RM0390](https://www.st.com/resource/en/reference_manual/dm00135183.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32f469` | ARM® Cortex®-M4F r0p1 | [RM0386](https://www.st.com/resource/en/reference_manual/dm00127514.pdf) | `adc` `dma` `exti` `gpio` `i2c` `spi` `tim` `uart`       |
//! | `stm32l4x1` | ARM® Cortex®-M4F r0p1 | [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940.pdf) | `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart`       |
//! | `stm32l4x2` | ARM® Cortex®-M4F r0p1 | [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940.pdf) | `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart`       |
//! | `stm32l4x3` | ARM® Cortex®-M4F r0p1 | [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940.pdf) | `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart`       |
//! | `stm32l4x5` | ARM® Cortex®-M4F r0p1 | [RM0351](https://www.st.com/resource/en/reference_manual/dm00083560.pdf) | `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart`       |
//! | `stm32l4x6` | ARM® Cortex®-M4F r0p1 | [RM0351](https://www.st.com/resource/en/reference_manual/dm00083560.pdf) | `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart`       |
//! | `stm32l4r5` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) | `adc` `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart` |
//! | `stm32l4s5` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) | `adc` `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart` |
//! | `stm32l4r7` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) | `adc` `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart` |
//! | `stm32l4s7` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) | `adc` `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart` |
//! | `stm32l4r9` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) | `adc` `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart` |
//! | `stm32l4s9` | ARM® Cortex®-M4F r0p1 | [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109.pdf) | `adc` `dma` `exti` `gpio` `i2c` `rtc` `spi` `tim` `uart` |
//! | `stm32l100` | ARM® Cortex®-M3 r1p1 | [RM0038](https://www.st.com/resource/en/reference_manual/rm0038-stm32l100xx-stm32l151xx-stm32l152xx-and-stm32l162xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf) | ``dma` `gpio` `spi` `tim` `uart` |
//! | `stm32l151` | ARM® Cortex®-M3 r1p1 | [RM0038](https://www.st.com/resource/en/reference_manual/rm0038-stm32l100xx-stm32l151xx-stm32l152xx-and-stm32l162xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf) | ``dma` `gpio` `spi` `tim` `uart` |
//! | `stm32l162` | ARM® Cortex®-M3 r1p1 | [RM0038](https://www.st.com/resource/en/reference_manual/rm0038-stm32l100xx-stm32l151xx-stm32l152xx-and-stm32l162xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf) | ``dma` `gpio` `spi` `tim` `uart` |
//! `stm32_mcu` config flag should be set at the application level according to
//! this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-stm32-map/0.14/)
//!
//! The API documentation intentionally skips auto-generated [`reg`] and [`thr`]
//! bindings. Otherwise it would use several gigabytes of space and would be
//! very slow to render in a browser. One should refer to the Reference Manual
//! instead. And to get an idea of what the API looks like on the Drone side,
//! look at the [`drone_cortexm::map`] module documentation.
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-stm32-map = { version = "0.14.0", features = [...] }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["drone-stm32-map/std"]
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![no_std]

pub mod periph;
pub mod reg;
pub mod thr;

pub use drone_stm32_map_pieces::stm32_reg_tokens;
