//! Basic timers.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic basic timer peripheral variant.
    pub trait BasicTimMap {}

    /// Generic basic timer peripheral.
    pub struct BasicTimPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            TIMEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            TIMRST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            TIMSMEN { RwRwRegFieldBit }
        }
    }

    TIM {
        CR1 {
            0x20 RwReg;
            CEN { RwRwRegFieldBit }
            UDIS { RwRwRegFieldBit }
            URS { RwRwRegFieldBit }
            OPM { RwRwRegFieldBit }
            ARPE { RwRwRegFieldBit }
            UIFREMAP { RwRwRegFieldBit }
        }
        CR2 {
            0x20 RwReg;
            MMS { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwReg;
            UIE { RwRwRegFieldBit }
            UDE { RwRwRegFieldBit }
        }
        SR {
            0x20 RwReg;
            UIF { RwRwRegFieldBit }
        }
        EGR {
            0x20 WoReg;
            UG { WoWoRegFieldBit }
        }
        CNT {
            0x20 RwReg;
            CNT { RwRwRegFieldBits }
            UIFCPY { RwRwRegFieldBit }
        }
        PSC {
            0x20 RwReg;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwReg;
            ARR { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_basic_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $timen:ident,
        $timrst:ident,
        $timsmen:ident,
        $tim:ident,
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl BasicTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::basic;

            RCC {
                BUSENR {
                    $busenr Shared;
                    TIMEN { $timen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    TIMRST { $timrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    TIMSMEN { $timsmen }
                }
            }

            TIM {
                $tim;
                CR1 {
                    CR1;
                    CEN { CEN }
                    UDIS { UDIS }
                    URS { URS }
                    OPM { OPM }
                    ARPE { ARPE }
                    UIFREMAP { UIFREMAP }
                }
                CR2 {
                    CR2;
                    MMS { MMS }
                }
                DIER {
                    DIER;
                    UIE { UIE }
                    UDE { UDE }
                }
                SR {
                    SR;
                    UIF { UIF }
                }
                EGR {
                    EGR;
                    UG { UG }
                }
                CNT {
                    CNT;
                    CNT { CNT }
                    UIFCPY { UIFCPY }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    ARR;
                    ARR { ARR }
                }
            }
        }
    };
}

map_basic_tim! {
    "Extracts TIM6 register tokens.",
    periph_tim6,
    "TIM6 peripheral variant.",
    Tim6,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM6EN,
    TIM6RST,
    TIM6LPEN,
    TIM6,
}

map_basic_tim! {
    "Extracts TIM7 register tokens.",
    periph_tim7,
    "TIM7 peripheral variant.",
    Tim7,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM7EN,
    TIM7RST,
    TIM7LPEN,
    TIM7,
}
