//! DMA channels.
//!
//! For STM32H7 series MCUs.
//!
//! In the STM32H7 series of MCUs channels are called streams which
//! are composed of channels. In order to maintain the same name as
//! other STM32 MCUs the streams here are called channels and their
//! mappings are exposed here. So instead of having a `DmaStreamMap`
//! we have a [`DmaChMap`].
//!
//! This also applies for the register names which on the STM32H7
//! series they follow the SxCR naming scheme where x is the stream
//! number, here we just use CxCR for all (so in the mapping it's CCR).

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMA channel peripheral variant.
    pub trait DmaChMap {
        /// DMA head peripheral variant.
        type DmaMap: super::super::DmaMap;
    }

    /// Generic DMA channel peripheral.
    pub struct DmaChPeriph;

    DMA {
        ISR {
            0x20 RoReg Shared;
            DMEIF { RoRoRegFieldBit }
            TEIF { RoRoRegFieldBit }
            HTIF { RoRoRegFieldBit }
            TCIF { RoRoRegFieldBit }
            FEIF { RoRoRegFieldBit }
        }
        IFCR {
            0x20 WoReg Shared;
            CFEIF { WoWoRegFieldBit }
            CDMEIF { WoWoRegFieldBit }
            CTEIF { WoWoRegFieldBit }
            CHTIF { WoWoRegFieldBit }
            CTCIF { WoWoRegFieldBit }
        }
        CCR {
            0x20 RwReg;
            EN { RwRwRegFieldBit }
            DMEIE { RwRwRegFieldBit }
            TEIE { RwRwRegFieldBit }
            HTIE { RwRwRegFieldBit }
            TCIE { RwRwRegFieldBit }
            PFCTRL { RwRwRegFieldBit }
            DIR { RwRwRegFieldBits }
            CIRC { RwRwRegFieldBit }
            PINC { RwRwRegFieldBit }
            MINC { RwRwRegFieldBit }
            PSIZE { RwRwRegFieldBits }
            MSIZE { RwRwRegFieldBits }
            PINCOS { RwRwRegFieldBit }
            PL { RwRwRegFieldBits }
            DBM { RwRwRegFieldBit }
            CT { RwRwRegFieldBit }
            TRBUFF { RwRwRegFieldBit }
            PBURST { RwRwRegFieldBits }
            MBURST { RwRwRegFieldBits }
        }
        CNDTR {
            0x20 RwReg;
            NDT { RwRwRegFieldBits }
        }
        CPAR {
            0x20 RwReg;
            PAR { RwRwRegFieldBits }
        }
        CM0AR {
            0x20 RwReg;
            M0A { RwRwRegFieldBits }
        }
        CM1AR {
            0x20 RwReg;
            M1A { RwRwRegFieldBits }
        }
        CFCR {
            0x20 RwReg;
            FTH { RwRwRegFieldBits }
            DMDIS { RwRwRegFieldBit }
            FS { RoRwRegFieldBits }
            FEIE { RwRwRegFieldBit }
        }
    }
}

macro_rules! map_dma_ch {
    (
        $dma_ch_macro_doc:expr,
        $dma_ch_macro:ident,
        $dma_ch_ty_doc:expr,
        $dma_ch_ty:ident,
        $dma_ty:ident,
        $dma:ident,
        $isr:ident,
        $dmeif:ident,
        $teif:ident,
        $htif:ident,
        $tcif:ident,
        $feif:ident,
        $ifcr:ident,
        $cfeif:ident,
        $cdmeif:ident,
        $cteif:ident,
        $chtif:ident,
        $ctcif:ident,
        $ccr:ident,
        $cndtr:ident,
        $cpar:ident,
        $cm0ar:ident,
        $cm1ar:ident,
        $cfcr:ident,
    ) => {
        periph::map! {
            #[doc = $dma_ch_macro_doc]
            pub macro $dma_ch_macro;

            #[doc = $dma_ch_ty_doc]
            pub struct $dma_ch_ty;

            impl DmaChMap for $dma_ch_ty {
                type DmaMap = super::super::$dma_ty;
            }

            drone_stm32_map_pieces::reg;
            crate::ch;

            DMA {
                $dma;
                ISR {
                    $isr Shared;
                    DMEIF { $dmeif }
                    TEIF { $teif }
                    HTIF { $htif }
                    TCIF { $tcif }
                    FEIF { $feif }
                }
                IFCR {
                    $ifcr Shared;
                    CFEIF { $cfeif }
                    CDMEIF { $cdmeif }
                    CTEIF { $cteif }
                    CHTIF { $chtif }
                    CTCIF { $ctcif }
                }
                CCR {
                    $ccr;
                    EN { EN }
                    DMEIE { DMEIE }
                    TEIE { TEIE }
                    HTIE { HTIE }
                    TCIE { TCIE }
                    PFCTRL { PFCTRL }
                    DIR { DIR }
                    CIRC { CIRC }
                    PINC { PINC }
                    MINC { MINC }
                    PSIZE { PSIZE }
                    MSIZE { MSIZE }
                    PINCOS { PINCOS }
                    PL { PL }
                    DBM { DBM }
                    CT { CT }
                    TRBUFF { TRBUFF }
                    PBURST { PBURST }
                    MBURST { MBURST }
                }
                CNDTR {
                    $cndtr;
                    NDT { NDT }
                }
                CPAR {
                    $cpar;
                    PAR { PAR }
                }
                CM0AR {
                    $cm0ar;
                    M0A { M0A }
                }
                CM1AR {
                    $cm1ar;
                    M1A { M1A }
                }
                CFCR {
                    $cfcr;
                    FTH { FTH }
                    DMDIS { DMDIS }
                    FS { FS }
                    FEIE { FEIE }
                }
            }
        }
    };
}

map_dma_ch! {
    "Extracts DMA1 channel 0 register tokens.",
    periph_dma1_ch0,
    "DMA1 channel 0 peripheral variant.",
    Dma1Ch0,
    Dma1,
    DMA1,
    LISR,
    DMEIF0,
    TEIF0,
    HTIF0,
    TCIF0,
    FEIF0,
    LIFCR,
    CFEIF0,
    CDMEIF0,
    CTEIF0,
    CHTIF0,
    CTCIF0,
    S0CR,
    S0NDTR,
    S0PAR,
    S0M0AR,
    S0M1AR,
    S0FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 1 register tokens.",
    periph_dma1_ch1,
    "DMA1 channel 1 peripheral variant.",
    Dma1Ch1,
    Dma1,
    DMA1,
    LISR,
    DMEIF1,
    TEIF1,
    HTIF1,
    TCIF1,
    FEIF1,
    LIFCR,
    CFEIF1,
    CDMEIF1,
    CTEIF1,
    CHTIF1,
    CTCIF1,
    S1CR,
    S1NDTR,
    S1PAR,
    S1M0AR,
    S1M1AR,
    S1FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 2 register tokens.",
    periph_dma1_ch2,
    "DMA1 channel 2 peripheral variant.",
    Dma1Ch2,
    Dma1,
    DMA1,
    LISR,
    DMEIF2,
    TEIF2,
    HTIF2,
    TCIF2,
    FEIF2,
    LIFCR,
    CFEIF2,
    CDMEIF2,
    CTEIF2,
    CHTIF2,
    CTCIF2,
    S2CR,
    S2NDTR,
    S2PAR,
    S2M0AR,
    S2M1AR,
    S2FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 3 register tokens.",
    periph_dma1_ch3,
    "DMA1 channel 3 peripheral variant.",
    Dma1Ch3,
    Dma1,
    DMA1,
    LISR,
    DMEIF3,
    TEIF3,
    HTIF3,
    TCIF3,
    FEIF3,
    LIFCR,
    CFEIF3,
    CDMEIF3,
    CTEIF3,
    CHTIF3,
    CTCIF3,
    S3CR,
    S3NDTR,
    S3PAR,
    S3M0AR,
    S3M1AR,
    S3FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 4 register tokens.",
    periph_dma1_ch4,
    "DMA1 channel 4 peripheral variant.",
    Dma1Ch4,
    Dma1,
    DMA1,
    HISR,
    DMEIF4,
    TEIF4,
    HTIF4,
    TCIF4,
    FEIF4,
    HIFCR,
    CFEIF4,
    CDMEIF4,
    CTEIF4,
    CHTIF4,
    CTCIF4,
    S4CR,
    S4NDTR,
    S4PAR,
    S4M0AR,
    S4M1AR,
    S4FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 5 register tokens.",
    periph_dma1_ch5,
    "DMA1 channel 5 peripheral variant.",
    Dma1Ch5,
    Dma1,
    DMA1,
    HISR,
    DMEIF5,
    TEIF5,
    HTIF5,
    TCIF5,
    FEIF5,
    HIFCR,
    CFEIF5,
    CDMEIF5,
    CTEIF5,
    CHTIF5,
    CTCIF5,
    S5CR,
    S5NDTR,
    S5PAR,
    S5M0AR,
    S5M1AR,
    S5FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 6 register tokens.",
    periph_dma1_ch6,
    "DMA1 channel 6 peripheral variant.",
    Dma1Ch6,
    Dma1,
    DMA1,
    HISR,
    DMEIF6,
    TEIF6,
    HTIF6,
    TCIF6,
    FEIF6,
    HIFCR,
    CFEIF6,
    CDMEIF6,
    CTEIF6,
    CHTIF6,
    CTCIF6,
    S6CR,
    S6NDTR,
    S6PAR,
    S6M0AR,
    S6M1AR,
    S6FCR,
}

map_dma_ch! {
    "Extracts DMA1 channel 7 register tokens.",
    periph_dma1_ch7,
    "DMA1 channel 7 peripheral variant.",
    Dma1Ch7,
    Dma1,
    DMA1,
    HISR,
    DMEIF7,
    TEIF7,
    HTIF7,
    TCIF7,
    FEIF7,
    HIFCR,
    CFEIF7,
    CDMEIF7,
    CTEIF7,
    CHTIF7,
    CTCIF7,
    S7CR,
    S7NDTR,
    S7PAR,
    S7M0AR,
    S7M1AR,
    S7FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 0 register tokens.",
    periph_dma2_ch0,
    "DMA2 channel 0 peripheral variant.",
    Dma2Ch0,
    Dma2,
    DMA2,
    LISR,
    DMEIF0,
    TEIF0,
    HTIF0,
    TCIF0,
    FEIF0,
    LIFCR,
    CFEIF0,
    CDMEIF0,
    CTEIF0,
    CHTIF0,
    CTCIF0,
    S0CR,
    S0NDTR,
    S0PAR,
    S0M0AR,
    S0M1AR,
    S0FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 1 register tokens.",
    periph_dma2_ch1,
    "DMA2 channel 1 peripheral variant.",
    Dma2Ch1,
    Dma2,
    DMA2,
    LISR,
    DMEIF1,
    TEIF1,
    HTIF1,
    TCIF1,
    FEIF1,
    LIFCR,
    CFEIF1,
    CDMEIF1,
    CTEIF1,
    CHTIF1,
    CTCIF1,
    S1CR,
    S1NDTR,
    S1PAR,
    S1M0AR,
    S1M1AR,
    S1FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 2 register tokens.",
    periph_dma2_ch2,
    "DMA2 channel 2 peripheral variant.",
    Dma2Ch2,
    Dma2,
    DMA2,
    LISR,
    DMEIF2,
    TEIF2,
    HTIF2,
    TCIF2,
    FEIF2,
    LIFCR,
    CFEIF2,
    CDMEIF2,
    CTEIF2,
    CHTIF2,
    CTCIF2,
    S2CR,
    S2NDTR,
    S2PAR,
    S2M0AR,
    S2M1AR,
    S2FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 3 register tokens.",
    periph_dma2_ch3,
    "DMA2 channel 3 peripheral variant.",
    Dma2Ch3,
    Dma2,
    DMA2,
    LISR,
    DMEIF3,
    TEIF3,
    HTIF3,
    TCIF3,
    FEIF3,
    LIFCR,
    CFEIF3,
    CDMEIF3,
    CTEIF3,
    CHTIF3,
    CTCIF3,
    S3CR,
    S3NDTR,
    S3PAR,
    S3M0AR,
    S3M1AR,
    S3FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 4 register tokens.",
    periph_dma2_ch4,
    "DMA2 channel 4 peripheral variant.",
    Dma2Ch4,
    Dma2,
    DMA2,
    HISR,
    DMEIF4,
    TEIF4,
    HTIF4,
    TCIF4,
    FEIF4,
    HIFCR,
    CFEIF4,
    CDMEIF4,
    CTEIF4,
    CHTIF4,
    CTCIF4,
    S4CR,
    S4NDTR,
    S4PAR,
    S4M0AR,
    S4M1AR,
    S4FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 5 register tokens.",
    periph_dma2_ch5,
    "DMA2 channel 5 peripheral variant.",
    Dma2Ch5,
    Dma2,
    DMA2,
    HISR,
    DMEIF5,
    TEIF5,
    HTIF5,
    TCIF5,
    FEIF5,
    HIFCR,
    CFEIF5,
    CDMEIF5,
    CTEIF5,
    CHTIF5,
    CTCIF5,
    S5CR,
    S5NDTR,
    S5PAR,
    S5M0AR,
    S5M1AR,
    S5FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 6 register tokens.",
    periph_dma2_ch6,
    "DMA2 channel 6 peripheral variant.",
    Dma2Ch6,
    Dma2,
    DMA2,
    HISR,
    DMEIF6,
    TEIF6,
    HTIF6,
    TCIF6,
    FEIF6,
    HIFCR,
    CFEIF6,
    CDMEIF6,
    CTEIF6,
    CHTIF6,
    CTCIF6,
    S6CR,
    S6NDTR,
    S6PAR,
    S6M0AR,
    S6M1AR,
    S6FCR,
}

map_dma_ch! {
    "Extracts DMA2 channel 7 register tokens.",
    periph_dma2_ch7,
    "DMA2 channel 7 peripheral variant.",
    Dma2Ch7,
    Dma2,
    DMA2,
    HISR,
    DMEIF7,
    TEIF7,
    HTIF7,
    TCIF7,
    FEIF7,
    HIFCR,
    CFEIF7,
    CDMEIF7,
    CTEIF7,
    CHTIF7,
    CTCIF7,
    S7CR,
    S7NDTR,
    S7PAR,
    S7M0AR,
    S7M1AR,
    S7FCR,
}
