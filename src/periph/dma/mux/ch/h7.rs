//! DMAMUX channels.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMAMUX channel peripheral variant.
    pub trait DmamuxChMap {}

    /// Generic DMAMUX channel peripheral.
    pub struct DmamuxChPeriph;

    DMAMUX {
        CCR {
            0x20 RwReg;
            SYNC_ID { RwRwRegFieldBits }
            NBREQ { RwRwRegFieldBits }
            SPOL { RwRwRegFieldBits }
            SE { RwRwRegFieldBit }
            EGE { RwRwRegFieldBit }
            SOIE { RwRwRegFieldBit }
            DMAREQ_ID { RwRwRegFieldBits }
        }
        CSR {
            0x20 RoReg Shared;
            SOF { RoRoRegFieldBit }
        }
        CFR {
            0x20 WoReg Shared;
            CSOF { WoWoRegFieldBit }
        }
    }
}

macro_rules! map_dmamux_ch {
    (
        $dmamux_ch_macro_doc:expr,
        $dmamux_ch_macro:ident,
        $dmamux_ch_ty_doc:expr,
        $dmamux_ch_ty:ident,
        $dmamux:ident,
        $ccr:ident,
        $sof:ident,
        $csof:ident,
    ) => {
        periph::map! {
            #[doc = $dmamux_ch_macro_doc]
            pub macro $dmamux_ch_macro;

            #[doc = $dmamux_ch_ty_doc]
            pub struct $dmamux_ch_ty;

            impl DmamuxChMap for $dmamux_ch_ty {}

            drone_stm32_map_pieces::reg;
            crate::mux::ch;

            DMAMUX {
                $dmamux;
                CCR {
                    $ccr;
                    SYNC_ID { SYNC_ID }
                    NBREQ { NBREQ }
                    SPOL { SPOL }
                    SE { SE }
                    EGE { EGE }
                    SOIE { SOIE }
                    DMAREQ_ID { DMAREQ_ID }
                }
                CSR {
                    CSR Shared;
                    SOF { $sof }
                }
                CFR {
                    CFR Shared;
                    CSOF { $csof }
                }
            }
        }
    };
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 0 register tokens.",
    periph_dmamux1_ch0,
    "DMAMUX1 channel 0 peripheral variant.",
    Dmamux1Ch0,
    DMAMUX1,
    C0CR,
    SOF0,
    CSOF0,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 1 register tokens.",
    periph_dmamux1_ch1,
    "DMAMUX1 channel 1 peripheral variant.",
    Dmamux1Ch1,
    DMAMUX1,
    C1CR,
    SOF1,
    CSOF1,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 2 register tokens.",
    periph_dmamux1_ch2,
    "DMAMUX1 channel 2 peripheral variant.",
    Dmamux1Ch2,
    DMAMUX1,
    C2CR,
    SOF2,
    CSOF2,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 3 register tokens.",
    periph_dmamux1_ch3,
    "DMAMUX1 channel 3 peripheral variant.",
    Dmamux1Ch3,
    DMAMUX1,
    C3CR,
    SOF3,
    CSOF3,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 4 register tokens.",
    periph_dmamux1_ch4,
    "DMAMUX1 channel 4 peripheral variant.",
    Dmamux1Ch4,
    DMAMUX1,
    C4CR,
    SOF4,
    CSOF4,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 5 register tokens.",
    periph_dmamux1_ch5,
    "DMAMUX1 channel 5 peripheral variant.",
    Dmamux1Ch5,
    DMAMUX1,
    C5CR,
    SOF5,
    CSOF5,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 6 register tokens.",
    periph_dmamux1_ch6,
    "DMAMUX1 channel 6 peripheral variant.",
    Dmamux1Ch6,
    DMAMUX1,
    C6CR,
    SOF6,
    CSOF6,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 7 register tokens.",
    periph_dmamux1_ch7,
    "DMAMUX1 channel 7 peripheral variant.",
    Dmamux1Ch7,
    DMAMUX1,
    C7CR,
    SOF7,
    CSOF7,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 8 register tokens.",
    periph_dmamux1_ch8,
    "DMAMUX1 channel 8 peripheral variant.",
    Dmamux1Ch8,
    DMAMUX1,
    C8CR,
    SOF8,
    CSOF8,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 9 register tokens.",
    periph_dmamux1_ch9,
    "DMAMUX1 channel 9 peripheral variant.",
    Dmamux1Ch9,
    DMAMUX1,
    C9CR,
    SOF9,
    CSOF9,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 10 register tokens.",
    periph_dmamux1_ch10,
    "DMAMUX1 channel 10 peripheral variant.",
    Dmamux1Ch10,
    DMAMUX1,
    C10CR,
    SOF10,
    CSOF10,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 11 register tokens.",
    periph_dmamux1_ch11,
    "DMAMUX1 channel 11 peripheral variant.",
    Dmamux1Ch11,
    DMAMUX1,
    C11CR,
    SOF11,
    CSOF11,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 12 peripheral varian tokens.",
    periph_dmamux1_ch12,
    "DMAMUX1 channel 12.",
    Dmamux1Ch12,
    DMAMUX1,
    C12CR,
    SOF12,
    CSOF12,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 13 register tokens.",
    periph_dmamux1_ch13,
    "DMAMUX1 channel 13 peripheral variant.",
    Dmamux1Ch13,
    DMAMUX1,
    C13CR,
    SOF13,
    CSOF13,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 14 register tokens.",
    periph_dmamux1_ch14,
    "DMAMUX1 channel 14 peripheral variant.",
    Dmamux1Ch14,
    DMAMUX1,
    C14CR,
    SOF14,
    CSOF14,
}

map_dmamux_ch! {
    "Extracts DMAMUX1 channel 15 register tokens.",
    periph_dmamux1_ch15,
    "DMAMUX1 channel 15 peripheral variant.",
    Dmamux1Ch15,
    DMAMUX1,
    C15CR,
    SOF15,
    CSOF15,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 0 register tokens.",
    periph_dmamux2_ch0,
    "DMAMUX2 channel 0 peripheral variant.",
    Dmamux2Ch0,
    DMAMUX2,
    C0CR,
    SOF0,
    CSOF0,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 1 register tokens.",
    periph_dmamux2_ch1,
    "DMAMUX2 channel 1 peripheral variant.",
    Dmamux2Ch1,
    DMAMUX2,
    C1CR,
    SOF1,
    CSOF1,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 2 register tokens.",
    periph_dmamux2_ch2,
    "DMAMUX2 channel 2 peripheral variant.",
    Dmamux2Ch2,
    DMAMUX2,
    C2CR,
    SOF2,
    CSOF2,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 3 register tokens.",
    periph_dmamux2_ch3,
    "DMAMUX2 channel 3 peripheral variant.",
    Dmamux2Ch3,
    DMAMUX2,
    C3CR,
    SOF3,
    CSOF3,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 4 register tokens.",
    periph_dmamux2_ch4,
    "DMAMUX2 channel 4 peripheral variant.",
    Dmamux2Ch4,
    DMAMUX2,
    C4CR,
    SOF4,
    CSOF4,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 5 register tokens.",
    periph_dmamux2_ch5,
    "DMAMUX2 channel 5 peripheral variant.",
    Dmamux2Ch5,
    DMAMUX2,
    C5CR,
    SOF5,
    CSOF5,
}

map_dmamux_ch! {
    "Extracts DMAMUX2 channel 6 register tokens.",
    periph_dmamux2_ch6,
    "DMAMUX2 channel 6 peripheral variant.",
    Dmamux2Ch6,
    DMAMUX2,
    C6CR,
    SOF6,
    CSOF6,
}
