//! Quad Serial Peripheral Interface.
//!
//! For STM32H7 series MCUs.

use drone_core::periph;

periph::singular! {
    /// Extracts QUADSPI register tokens.
    pub macro periph_quadspi;

    /// Quad Serial Peripheral Interface peripheral.
    pub struct QuadSpiPeriph;

    drone_stm32_map_pieces::reg;
    crate::quad;

    RCC {
        AHB3ENR {
            QSPIEN;
        }
        AHB3RSTR {
            QSPIRST;
        }
        AHB3LPENR {
            QSPILPEN;
        }
        D1CCIPR {
            QSPISEL;
        }
    }

    QUADSPI {
        CR;
        DCR;
        SR;
        FCR;
        DLR;
        CCR;
        AR;
        ABR;
        DR;
        PSMKR;
        PSMAR;
        PIR;
        LPTR;
    }
}
