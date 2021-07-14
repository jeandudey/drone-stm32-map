//! DMAMUX request generators.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic DMAMUX request generator peripheral variant.
    pub trait DmamuxRgMap {}

    /// Generic DMAMUX request generator peripheral.
    pub struct DmamuxRgPeriph;

    DMAMUX {
        RGCR {
            0x20 RwReg;
            SIG_ID { RwRwRegFieldBits }
            OIE { RwRwRegFieldBit }
            GE { RwRwRegFieldBit }
            GPOL { RwRwRegFieldBits }
            GNBREQ { RwRwRegFieldBits }
        }
        RGSR {
            0x20 RoReg Shared;
            OF { RoRoRegFieldBit }
        }
        RGCFR {
            0x20 WoReg Shared;
            COF { WoWoRegFieldBit }
        }
    }
}

macro_rules! map_dmamux_rg {
    (
        $dmamux_rg_macro_doc:expr,
        $dmamux_rg_macro:ident,
        $dmamux_rg_ty_doc:expr,
        $dmamux_rg_ty:ident,
        $dmamux:ident,
        $rgcr:ident,
        $of:ident,
        $cof:ident,
    ) => {
        periph::map! {
            #[doc = $dmamux_rg_macro_doc]
            pub macro $dmamux_rg_macro;

            #[doc = $dmamux_rg_ty_doc]
            pub struct $dmamux_rg_ty;

            impl DmamuxRgMap for $dmamux_rg_ty {}

            drone_stm32_map_pieces::reg;
            crate::mux::rg;

            DMAMUX {
                $dmamux;
                RGCR {
                    $rgcr;
                    GNBREQ { GNBREQ }
                    GPOL { GPOL }
                    GE { GE }
                    OIE { OIE }
                    SIG_ID { SIG_ID }
                }
                RGSR {
                    RGSR Shared;
                    OF { $of }
                }
                RGCFR {
                    RGCFR Shared;
                    COF { $cof }
                }
            }
        }
    };
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 0 register tokens.",
    periph_dmamux1_rg0,
    "DMAMUX1 request generator 0 peripheral.",
    Dmamux1Rg0,
    DMAMUX1,
    RG0CR,
    OF0,
    COF0,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 1 register tokens.",
    periph_dmamux1_rg1,
    "DMAMUX1 request generator 1 peripheral.",
    Dmamux1Rg1,
    DMAMUX1,
    RG1CR,
    OF1,
    COF1,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 2 register tokens.",
    periph_dmamux1_rg2,
    "DMAMUX1 request generator 2 peripheral.",
    Dmamux1Rg2,
    DMAMUX1,
    RG2CR,
    OF2,
    COF2,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 3 register tokens.",
    periph_dmamux1_rg3,
    "DMAMUX1 request generator 3 peripheral.",
    Dmamux1Rg3,
    DMAMUX1,
    RG3CR,
    OF3,
    COF3,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 4 register tokens.",
    periph_dmamux1_rg4,
    "DMAMUX1 request generator 4 peripheral.",
    Dmamux1Rg4,
    DMAMUX1,
    RG4CR,
    OF4,
    COF4,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 5 register tokens.",
    periph_dmamux1_rg5,
    "DMAMUX1 request generator 5 peripheral.",
    Dmamux1Rg5,
    DMAMUX1,
    RG5CR,
    OF5,
    COF5,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 6 register tokens.",
    periph_dmamux1_rg6,
    "DMAMUX1 request generator 6 peripheral.",
    Dmamux1Rg6,
    DMAMUX1,
    RG6CR,
    OF6,
    COF6,
}

map_dmamux_rg! {
    "Extracts DMAMUX1 request generator 7 register tokens.",
    periph_dmamux1_rg7,
    "DMAMUX1 request generator 7 peripheral.",
    Dmamux1Rg7,
    DMAMUX1,
    RG7CR,
    OF7,
    COF7,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 0 register tokens.",
    periph_dmamux2_rg0,
    "DMAMUX2 request generator 0 peripheral.",
    Dmamux2Rg0,
    DMAMUX1,
    RG0CR,
    OF0,
    COF0,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 1 register tokens.",
    periph_dmamux2_rg1,
    "DMAMUX2 request generator 1 peripheral.",
    Dmamux2Rg1,
    DMAMUX2,
    RG1CR,
    OF1,
    COF1,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 2 register tokens.",
    periph_dmamux2_rg2,
    "DMAMUX2 request generator 2 peripheral.",
    Dmamux2Rg2,
    DMAMUX2,
    RG2CR,
    OF2,
    COF2,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 3 register tokens.",
    periph_dmamux2_rg3,
    "DMAMUX2 request generator 3 peripheral.",
    Dmamux2Rg3,
    DMAMUX2,
    RG3CR,
    OF3,
    COF3,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 4 register tokens.",
    periph_dmamux2_rg4,
    "DMAMUX2 request generator 4 peripheral.",
    Dmamux2Rg4,
    DMAMUX2,
    RG4CR,
    OF4,
    COF4,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 5 register tokens.",
    periph_dmamux2_rg5,
    "DMAMUX2 request generator 5 peripheral.",
    Dmamux2Rg5,
    DMAMUX2,
    RG5CR,
    OF5,
    COF5,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 6 register tokens.",
    periph_dmamux2_rg6,
    "DMAMUX2 request generator 6 peripheral.",
    Dmamux2Rg6,
    DMAMUX2,
    RG6CR,
    OF6,
    COF6,
}

map_dmamux_rg! {
    "Extracts DMAMUX2 request generator 7 register tokens.",
    periph_dmamux2_rg7,
    "DMAMUX2 request generator 7 peripheral.",
    Dmamux2Rg7,
    DMAMUX2,
    RG7CR,
    OF7,
    COF7,
}
