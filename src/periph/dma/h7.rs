//! Direct Memory Access.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMA head peripheral variant.
    pub trait DmaMap {}

    /// Generic DMA head peripheral.
    pub struct DmaPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            DMAEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            DMARST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            DMALPEN { RwRwRegFieldBit }
        }
    }
}

macro_rules! map_dma {
    (
        $dma_macro_doc:expr,
        $dma_macro:ident,
        $dma_ty_doc:expr,
        $dma_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $buslpenr:ident,
        $dmaen:ident,
        $dmarst:ident,
        $dmalpen:ident,
    ) => {
        periph::map! {
            #[doc = $dma_macro_doc]
            pub macro $dma_macro;

            #[doc = $dma_ty_doc]
            pub struct $dma_ty;

            impl DmaMap for $dma_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    DMAEN { $dmaen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    DMARST { $dmarst }
                }
                BUSSMENR {
                    $buslpenr Shared;
                    DMALPEN { $dmalpen }
                }
            }
        }
    };
}

map_dma! {
    "Extracts DMA1 head register tokens.",
    periph_dma1,
    "DMA1 head peripheral variant.",
    Dma1,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    DMA1EN,
    DMA1RST,
    DMA1LPEN,
}

map_dma! {
    "Extracts DMA2 head register tokens.",
    periph_dma2,
    "DMA2 head peripheral variant.",
    Dma2,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    DMA2EN,
    DMA2RST,
    DMA2LPEN,
}
