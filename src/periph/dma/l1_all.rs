//! Direct Memory Access.
//!
//! For STM32L1 series of ultra-low-power MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMA head peripheral variant.
    pub trait DmaMap {}

    /// Generic DMA head peripheral.
    pub struct DmaPeriph;

    RCC {
        AHBENR {
            0x20 RwRegBitBand Shared;
            DMAEN { RwRwRegFieldBitBand }
        }
        AHBRSTR {
            0x20 RwRegBitBand Shared;
            DMARST { RwRwRegFieldBitBand }
        }
        AHBLPENR {
            0x20 RwRegBitBand Shared;
            DMALPEN { RwRwRegFieldBitBand }
        }
    }
}

macro_rules! map_dma {
    (
        $dma_macro_doc:expr,
        $dma_macro:ident,
        $dma_ty_doc:expr,
        $dma_ty:ident,
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
                AHBENR {
                    AHBENR Shared;
                    DMAEN { $dmaen }
                }
                AHBRSTR {
                    AHBRSTR Shared;
                    DMARST { $dmarst }
                }
                AHBLPENR {
                    AHBLPENR Shared;
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
    DMA1EN,
    DMA1RST,
    DMA1LPEN,
}

map_dma! {
    "Extracts DMA2 head register tokens.",
    periph_dma2,
    "DMA2 head peripheral variant.",
    Dma2,
    DMA2EN,
    DMA2RST,
    DMA2LPEN,
}