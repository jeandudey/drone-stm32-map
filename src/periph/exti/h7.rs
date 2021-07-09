//! Extended interrupts and events controller.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic EXTI peripheral variant.
    pub trait ExtiMap {}

    /// Generic EXTI peripheral.
    pub struct ExtiPeriph;

    SYSCFG {
        EXTICR {
            0x20 RwReg Shared;
            EXTI { RwRwRegFieldBits Option }
        }
    }

    EXTI {
        RTSR {
            0x20 RwReg Shared;
            TR { RwRwRegFieldBit Option }
        }
        FTSR {
            0x20 RwReg Shared;
            TR { RwRwRegFieldBit Option }
        }
        SWIER {
            0x20 RwReg Shared;
            SWIER { RwRwRegFieldBit Option }
        }
        D3PMR {
            0x20 RwReg Shared;
            MR { RwRwRegFieldBit Option }
        }
        D3PCR {
            0x20 RwReg Shared;
            PCS { RwRwRegFieldBits Option }
        }
        CPUIMR {
            0x20 RwReg Shared;
            MR { RwRwRegFieldBit }
        }
        CPUEMR {
            0x20 RwReg Shared;
            MR { RwRwRegFieldBit }
        }
        CPUPR {
            0x20 RwReg Shared;
            PR { RwRwRegFieldBit Option }
        }
    }
}

macro_rules! map_exti {
    (
        $exti_macro_doc:expr,
        $exti_macro:ident,
        $exti_ty_doc:expr,
        $exti_ty:ident,
        $exticr:ident,
        $rtsr:ident,
        $ftsr:ident,
        $swier:ident,
        $d3pmr:ident,
        $d3pcr:ident,
        $cpuimr:ident,
        $cpuemr:ident,
        $cpupr:ident,
        $cpumr_mr:ident,
        ($($exti:ident)?),
        ($($tr:ident)?),
        ($($swier_swier:ident)?),
        ($($d3pmr_mr:ident)?),
        ($($d3pcr_pcs:ident)?),
        ($($cpupr_pr:ident)?),
    ) => {
        periph::map! {
            #[doc = $exti_macro_doc]
            pub macro $exti_macro;

            #[doc = $exti_ty_doc]
            pub struct $exti_ty;

            impl ExtiMap for $exti_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            SYSCFG {
                EXTICR {
                    $exticr Shared;
                    EXTI { $($exti Option)* }
                }
            }

            EXTI {
                RTSR {
                    $rtsr Shared;
                    TR { $($tr Option)* }
                }
                FTSR {
                    $ftsr Shared;
                    TR { $($tr Option)* }
                }
                SWIER {
                    $swier Shared;
                    SWIER { $($swier_swier Option)* }
                }
                D3PMR {
                    $d3pmr Shared;
                    MR { $($d3pmr_mr Option)* }
                }
                D3PCR {
                    $d3pcr Shared;
                    PCS { $($d3pcr_pcs Option)* }
                }
                CPUIMR {
                    $cpuimr Shared;
                    MR { $cpumr_mr }
                }
                CPUEMR {
                    $cpuemr Shared;
                    MR { $cpumr_mr }
                }
                CPUPR {
                    $cpupr Shared;
                    PR { $($cpupr_pr Option)* }
                }
            }
        }
    };
}

map_exti! {
    "Extracts EXTI Line 0 register tokens.",
    periph_exti0,
    "EXTI Line 0 peripheral variant.",
    Exti0,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR0,
    (EXTI0),
    (TR0),
    (SWIER0),
    (MR0),
    (PCS0),
    (PR0),
}

map_exti! {
    "Extracts EXTI Line 1 register tokens.",
    periph_exti1,
    "EXTI Line 1 peripheral variant.",
    Exti1,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR1,
    (EXTI1),
    (TR1),
    (SWIER1),
    (MR1),
    (PCS1),
    (PR1),
}

map_exti! {
    "Extracts EXTI Line 2 register tokens.",
    periph_exti2,
    "EXTI Line 2 peripheral variant.",
    Exti2,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR2,
    (EXTI2),
    (TR2),
    (SWIER2),
    (MR2),
    (PCS2),
    (PR2),
}

map_exti! {
    "Extracts EXTI Line 3 register tokens.",
    periph_exti3,
    "EXTI Line 3 peripheral variant.",
    Exti3,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR3,
    (EXTI3),
    (TR3),
    (SWIER3),
    (MR3),
    (PCS3),
    (PR3),
}

map_exti! {
    "Extracts EXTI Line 4 register tokens.",
    periph_exti4,
    "EXTI Line 4 peripheral variant.",
    Exti4,
    EXTICR2,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR4,
    (EXTI4),
    (TR4),
    (SWIER4),
    (MR4),
    (PCS4),
    (PR4),
}

map_exti! {
    "Extracts EXTI Line 5 register tokens.",
    periph_exti5,
    "EXTI Line 5 peripheral variant.",
    Exti5,
    EXTICR2,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR5,
    (EXTI5),
    (TR5),
    (SWIER5),
    (MR5),
    (PCS5),
    (PR5),
}

map_exti! {
    "Extracts EXTI Line 6 register tokens.",
    periph_exti6,
    "EXTI Line 6 peripheral variant.",
    Exti6,
    EXTICR2,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR6,
    (EXTI6),
    (TR6),
    (SWIER6),
    (MR6),
    (PCS6),
    (PR6),
}

map_exti! {
    "Extracts EXTI Line 7 register tokens.",
    periph_exti7,
    "EXTI Line 7 peripheral variant.",
    Exti7,
    EXTICR2,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR7,
    (EXTI7),
    (TR7),
    (SWIER7),
    (MR7),
    (PCS7),
    (PR7),
}

map_exti! {
    "Extracts EXTI Line 8 register tokens.",
    periph_exti8,
    "EXTI Line 8 peripheral variant.",
    Exti8,
    EXTICR3,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR8,
    (EXTI8),
    (TR8),
    (SWIER8),
    (MR8),
    (PCS8),
    (PR8),
}

map_exti! {
    "Extracts EXTI Line 9 register tokens.",
    periph_exti9,
    "EXTI Line 9 peripheral variant.",
    Exti9,
    EXTICR3,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR9,
    (EXTI9),
    (TR9),
    (SWIER9),
    (MR9),
    (PCS9),
    (PR9),
}

map_exti! {
    "Extracts EXTI Line 10 register tokens.",
    periph_exti10,
    "EXTI Line 10 peripheral variant.",
    Exti10,
    EXTICR3,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR10,
    (EXTI10),
    (TR10),
    (SWIER10),
    (MR10),
    (PCS10),
    (PR10),
}

map_exti! {
    "Extracts EXTI Line 11 register tokens.",
    periph_exti11,
    "EXTI Line 11 peripheral variant.",
    Exti11,
    EXTICR3,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR11,
    (EXTI11),
    (TR11),
    (SWIER11),
    (MR11),
    (PCS11),
    (PR11),
}

map_exti! {
    "Extracts EXTI Line 12 register tokens.",
    periph_exti12,
    "EXTI Line 12 peripheral variant.",
    Exti12,
    EXTICR4,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR12,
    (EXTI12),
    (TR12),
    (SWIER12),
    (MR12),
    (PCS12),
    (PR12),
}

map_exti! {
    "Extracts EXTI Line 13 register tokens.",
    periph_exti13,
    "EXTI Line 13 peripheral variant.",
    Exti13,
    EXTICR4,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR13,
    (EXTI13),
    (TR13),
    (SWIER13),
    (MR13),
    (PCS13),
    (PR13),
}

map_exti! {
    "Extracts EXTI Line 14 register tokens.",
    periph_exti14,
    "EXTI Line 14 peripheral variant.",
    Exti14,
    EXTICR4,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR14,
    (EXTI14),
    (TR14),
    (SWIER14),
    (MR14),
    (PCS14),
    (PR14),
}

map_exti! {
    "Extracts EXTI Line 15 register tokens.",
    periph_exti15,
    "EXTI Line 15 peripheral variant.",
    Exti15,
    EXTICR4,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR15,
    (EXTI15),
    (TR15),
    (SWIER15),
    (MR15),
    (),
    (PR15),
}

map_exti! {
    "Extracts EXTI Line 16 register tokens.",
    periph_exti16,
    "EXTI Line 16 peripheral variant.",
    Exti16,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR16,
    (),
    (TR16),
    (SWIER16),
    (),
    (),
    (PR16),
}

map_exti! {
    "Extracts EXTI Line 17 register tokens.",
    periph_exti17,
    "EXTI Line 17 peripheral variant.",
    Exti17,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR17,
    (),
    (TR17),
    (SWIER17),
    (),
    (),
    (PR17),
}

map_exti! {
    "Extracts EXTI Line 18 register tokens.",
    periph_exti18,
    "EXTI Line 18 peripheral variant.",
    Exti18,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR18,
    (),
    (TR18),
    (SWIER18),
    (),
    (),
    (PR18),
}

map_exti! {
    "Extracts EXTI Line 19 register tokens.",
    periph_exti19,
    "EXTI Line 19 peripheral variant.",
    Exti19,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1H,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR19,
    (),
    (TR19),
    (SWIER19),
    (MR19),
    (PCS19),
    (PR19),
}

map_exti! {
    "Extracts EXTI Line 20 register tokens.",
    periph_exti20,
    "EXTI Line 20 peripheral variant.",
    Exti20,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1H,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR20,
    (),
    (TR20),
    (SWIER20),
    (MR20),
    (PCS20),
    (PR20),
}

map_exti! {
    "Extracts EXTI Line 21 register tokens.",
    periph_exti21,
    "EXTI Line 21 peripheral variant.",
    Exti21,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1H,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR21,
    (),
    (TR21),
    (SWIER21),
    (MR21),
    (PCS21),
    (PR21),
}

map_exti! {
    "Extracts EXTI Line 22 register tokens.",
    periph_exti22,
    "EXTI Line 22 peripheral variant.",
    Exti22,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR22,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 23 register tokens.",
    periph_exti23,
    "EXTI Line 23 peripheral variant.",
    Exti23,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR23,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 24 register tokens.",
    periph_exti24,
    "EXTI Line 24 peripheral variant.",
    Exti24,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR24,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 25 register tokens.",
    periph_exti25,
    "EXTI Line 25 peripheral variant.",
    Exti25,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1H,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR25,
    (),
    (),
    (),
    (MR25),
    (PCS25),
    (),
}

map_exti! {
    "Extracts EXTI Line 26 register tokens.",
    periph_exti26,
    "EXTI Line 26 peripheral variant.",
    Exti26,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR26,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 27 register tokens.",
    periph_exti27,
    "EXTI Line 27 peripheral variant.",
    Exti27,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR27,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 28 register tokens.",
    periph_exti28,
    "EXTI Line 28 peripheral variant.",
    Exti28,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR28,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 29 register tokens.",
    periph_exti29,
    "EXTI Line 29 peripheral variant.",
    Exti29,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR29,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 30 register tokens.",
    periph_exti30,
    "EXTI Line 30 peripheral variant.",
    Exti30,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR30,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 31 register tokens.",
    periph_exti31,
    "EXTI Line 31 peripheral variant.",
    Exti31,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR1,
    CPUEMR1,
    CPUPR1,
    MR31,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 32 register tokens.",
    periph_exti32,
    "EXTI Line 32 peripheral variant.",
    Exti32,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR32,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 33 register tokens.",
    periph_exti33,
    "EXTI Line 33 peripheral variant.",
    Exti33,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR33,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 34 register tokens.",
    periph_exti34,
    "EXTI Line 34 peripheral variant.",
    Exti34,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR34,
    (),
    (),
    (),
    (MR34),
    (PCS34),
    (),
}

map_exti! {
    "Extracts EXTI Line 35 register tokens.",
    periph_exti35,
    "EXTI Line 35 peripheral variant.",
    Exti35,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR35,
    (),
    (),
    (),
    (MR35),
    (PCS35),
    (),
}

map_exti! {
    "Extracts EXTI Line 36 register tokens.",
    periph_exti36,
    "EXTI Line 36 peripheral variant.",
    Exti36,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR36,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 37 register tokens.",
    periph_exti37,
    "EXTI Line 37 peripheral variant.",
    Exti37,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR37,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 38 register tokens.",
    periph_exti38,
    "EXTI Line 38 peripheral variant.",
    Exti38,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR38,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 39 register tokens.",
    periph_exti39,
    "EXTI Line 39 peripheral variant.",
    Exti39,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR39,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 40 register tokens.",
    periph_exti40,
    "EXTI Line 40 peripheral variant.",
    Exti40,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR40,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 41 register tokens.",
    periph_exti41,
    "EXTI Line 41 peripheral variant.",
    Exti41,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR41,
    (),
    (),
    (),
    (MR41),
    (PCS41),
    (),
}

map_exti! {
    "Extracts EXTI Line 42 register tokens.",
    periph_exti42,
    "EXTI Line 42 peripheral variant.",
    Exti42,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR42,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 43 register tokens.",
    periph_exti43,
    "EXTI Line 43 peripheral variant.",
    Exti43,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR43,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 44 register tokens.",
    periph_exti44,
    "EXTI Line 44 peripheral variant.",
    Exti44,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR44,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 46 register tokens.",
    periph_exti46,
    "EXTI Line 46 peripheral variant.",
    Exti46,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR46,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 48 register tokens.",
    periph_exti48,
    "EXTI Line 48 peripheral variant.",
    Exti48,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2H,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR48,
    (),
    (),
    (),
    (MR48),
    (PCS48),
    (),
}

map_exti! {
    "Extracts EXTI Line 49 register tokens.",
    periph_exti49,
    "EXTI Line 49 peripheral variant.",
    Exti49,
    EXTICR1,
    RTSR2,
    FTSR2,
    SWIER2,
    D3PMR2,
    D3PCR2H,
    CPUIMR2,
    CPUEMR2,
    CPUPR2,
    MR49,
    (),
    (TR49),
    (SWIER49),
    (MR49),
    (PCS49),
    (PR49),
}

map_exti! {
    "Extracts EXTI Line 50 register tokens.",
    periph_exti50,
    "EXTI Line 50 peripheral variant.",
    Exti50,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2H,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR50,
    (),
    (),
    (),
    (MR50),
    (PCS50),
    (),
}

map_exti! {
    "Extracts EXTI Line 51 register tokens.",
    periph_exti51,
    "EXTI Line 51 peripheral variant.",
    Exti51,
    EXTICR1,
    RTSR2,
    FTSR2,
    SWIER2,
    D3PMR2,
    D3PCR2H,
    CPUIMR2,
    CPUEMR2,
    CPUPR2,
    MR51,
    (),
    (TR51),
    (SWIER51),
    (MR51),
    (PCS51),
    (PR51),
}

map_exti! {
    "Extracts EXTI Line 52 register tokens.",
    periph_exti52,
    "EXTI Line 52 peripheral variant.",
    Exti52,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2H,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR52,
    (),
    (),
    (),
    (MR52),
    (PCS52),
    (),
}

map_exti! {
    "Extracts EXTI Line 53 register tokens.",
    periph_exti53,
    "EXTI Line 53 peripheral variant.",
    Exti53,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR2,
    D3PCR2H,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR53,
    (),
    (),
    (),
    (MR53),
    (PCS53),
    (),
}

map_exti! {
    "Extracts EXTI Line 54 register tokens.",
    periph_exti54,
    "EXTI Line 54 peripheral variant.",
    Exti54,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR54,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 55 register tokens.",
    periph_exti55,
    "EXTI Line 55 peripheral variant.",
    Exti55,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR55,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 56 register tokens.",
    periph_exti56,
    "EXTI Line 56 peripheral variant.",
    Exti56,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR56,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 57 register tokens.",
    periph_exti57,
    "EXTI Line 57 peripheral variant.",
    Exti57,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR57,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 58 register tokens.",
    periph_exti58,
    "EXTI Line 58 peripheral variant.",
    Exti58,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR58,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 59 register tokens.",
    periph_exti59,
    "EXTI Line 59 peripheral variant.",
    Exti59,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR59,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 60 register tokens.",
    periph_exti60,
    "EXTI Line 60 peripheral variant.",
    Exti60,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR60,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 61 register tokens.",
    periph_exti61,
    "EXTI Line 61 peripheral variant.",
    Exti61,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR61,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 62 register tokens.",
    periph_exti62,
    "EXTI Line 62 peripheral variant.",
    Exti62,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR62,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 63 register tokens.",
    periph_exti63,
    "EXTI Line 63 peripheral variant.",
    Exti63,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR2,
    CPUEMR2,
    CPUPR1,
    MR63,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 64 register tokens.",
    periph_exti64,
    "EXTI Line 64 peripheral variant.",
    Exti64,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR64,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 65 register tokens.",
    periph_exti65,
    "EXTI Line 65 peripheral variant.",
    Exti65,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR65,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 66 register tokens.",
    periph_exti66,
    "EXTI Line 66 peripheral variant.",
    Exti66,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR66,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 67 register tokens.",
    periph_exti67,
    "EXTI Line 67 peripheral variant.",
    Exti67,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR67,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 68 register tokens.",
    periph_exti68,
    "EXTI Line 68 peripheral variant.",
    Exti68,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR68,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 69 register tokens.",
    periph_exti69,
    "EXTI Line 69 peripheral variant.",
    Exti69,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR69,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 70 register tokens.",
    periph_exti70,
    "EXTI Line 70 peripheral variant.",
    Exti70,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR70,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 71 register tokens.",
    periph_exti71,
    "EXTI Line 71 peripheral variant.",
    Exti71,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR71,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 72 register tokens.",
    periph_exti72,
    "EXTI Line 72 peripheral variant.",
    Exti72,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR72,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 73 register tokens.",
    periph_exti73,
    "EXTI Line 73 peripheral variant.",
    Exti73,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR73,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 74 register tokens.",
    periph_exti74,
    "EXTI Line 74 peripheral variant.",
    Exti74,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR74,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 75 register tokens.",
    periph_exti75,
    "EXTI Line 75 peripheral variant.",
    Exti75,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR75,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 76 register tokens.",
    periph_exti76,
    "EXTI Line 76 peripheral variant.",
    Exti76,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR76,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 77 register tokens.",
    periph_exti77,
    "EXTI Line 77 peripheral variant.",
    Exti77,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR77,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 78 register tokens.",
    periph_exti78,
    "EXTI Line 78 peripheral variant.",
    Exti78,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR78,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 79 register tokens.",
    periph_exti79,
    "EXTI Line 79 peripheral variant.",
    Exti79,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR79,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 80 register tokens.",
    periph_exti80,
    "EXTI Line 80 peripheral variant.",
    Exti80,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER1,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR3,
    MR80,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 82 register tokens.",
    periph_exti82,
    "EXTI Line 82 peripheral variant.",
    Exti82,
    EXTICR1,
    RTSR3,
    FTSR3,
    SWIER3,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR3,
    MR82,
    (),
    (TR82),
    (SWIER82),
    (),
    (),
    (PR82),
}

map_exti! {
    "Extracts EXTI Line 84 register tokens.",
    periph_exti84,
    "EXTI Line 84 peripheral variant.",
    Exti84,
    EXTICR1,
    RTSR3,
    FTSR3,
    SWIER3,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR3,
    MR84,
    (),
    (TR84),
    (SWIER84),
    (),
    (),
    (PR84),
}

map_exti! {
    "Extracts EXTI Line 85 register tokens.",
    periph_exti85,
    "EXTI Line 85 peripheral variant.",
    Exti85,
    EXTICR1,
    RTSR3,
    FTSR3,
    SWIER3,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR3,
    MR85,
    (),
    (TR85),
    (SWIER85),
    (),
    (),
    (PR85),
}

map_exti! {
    "Extracts EXTI Line 86 register tokens.",
    periph_exti86,
    "EXTI Line 86 peripheral variant.",
    Exti86,
    EXTICR1,
    RTSR3,
    FTSR3,
    SWIER3,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR3,
    MR86,
    (),
    (TR86),
    (SWIER86),
    (),
    (),
    (PR86),
}

map_exti! {
    "Extracts EXTI Line 87 register tokens.",
    periph_exti87,
    "EXTI Line 87 peripheral variant.",
    Exti87,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER3,
    D3PMR1,
    D3PCR1L,
    CPUIMR3,
    CPUEMR3,
    CPUPR1,
    MR87,
    (),
    (),
    (),
    (),
    (),
    (),
}

map_exti! {
    "Extracts EXTI Line 88 register tokens.",
    periph_exti88,
    "EXTI Line 88 peripheral variant.",
    Exti88,
    EXTICR1,
    RTSR1,
    FTSR1,
    SWIER3,
    D3PMR3,
    D3PCR3H,
    CPUIMR3,
    CPUEMR3,
    CPUPR3,
    MR88,
    (),
    (),
    (),
    (MR88),
    (PCS88),
    (),
}
