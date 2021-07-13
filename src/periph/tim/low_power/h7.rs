//! Low-power timers.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic low-power timer peripheral variant.
    pub trait LowPowerTimMap {}

    /// Generic low-power timer peripheral.
    pub struct LowPowerTimPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            LPTIMEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            LPTIMRST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            LPTIMSMEN { RwRwRegFieldBit }
        }
        DCCIPR {
            0x20 RwReg Shared;
            LPTIMSEL { RwRwRegFieldBits }
        }
    }

    LPTIM {
        ISR {
            0x20 RoReg;
            CMPM { RoRoRegFieldBit }
            ARRM { RoRoRegFieldBit }
            EXTTRIG { RoRoRegFieldBit }
            CMPOK { RoRoRegFieldBit }
            ARROK { RoRoRegFieldBit }
            UP { RoRoRegFieldBit }
            DOWN { RoRoRegFieldBit }
        }
        ICR {
            0x20 WoReg;
            CMPMCF { WoWoRegFieldBit }
            ARRMCF { WoWoRegFieldBit }
            EXTTRIGCF { WoWoRegFieldBit }
            CMPOKCF { WoWoRegFieldBit }
            ARROKCF { WoWoRegFieldBit }
            UPCF { WoWoRegFieldBit }
            DOWNCF { WoWoRegFieldBit }
        }
        IER {
            0x20 RwReg;
            CMPMIE { RwRwRegFieldBit }
            ARRMIE { RwRwRegFieldBit }
            EXTTRIGIE { RwRwRegFieldBit }
            CMPOKIE { RwRwRegFieldBit }
            ARROKIE { RwRwRegFieldBit }
            UPIE { RwRwRegFieldBit }
            DOWNIE { RwRwRegFieldBit }
        }
        CFGR {
            0x20 RwReg;
            CKSEL { RwRwRegFieldBit }
            CKPOL { RwRwRegFieldBits }
            CKFLT { RwRwRegFieldBits }
            TRGFLT { RwRwRegFieldBits }
            PRESC { RwRwRegFieldBits }
            TRIGSEL { RwRwRegFieldBits }
            TRIGEN { RwRwRegFieldBits }
            TIMOUT { RwRwRegFieldBit }
            WAVE { RwRwRegFieldBit }
            WAVPOL { RwRwRegFieldBit }
            PRELOAD { RwRwRegFieldBit }
            COUNTMODE { RwRwRegFieldBit }
            ENC { RwRwRegFieldBit }
        }
        CR {
            0x20 RwReg;
            ENABLE { RwRwRegFieldBit }
            SNGSTRT { RwRwRegFieldBit }
            CNTSTRT { RwRwRegFieldBit }
            COUNTRST { RwRwRegFieldBit }
            RSTARE { RwRwRegFieldBit }
        }
        CMP {
            0x20 RwReg;
            CMP { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwReg;
            ARR { RwRwRegFieldBits }
        }
        CNT {
            0x20 RoReg;
            CNT { RoRoRegFieldBits }
        }
        CFGR2 {
            0x20 RwReg;
            IN1SEL { RwRwRegFieldBits }
            IN2SEL { RwRwRegFieldBits Option }
        }
    }
}

macro_rules! map_low_power_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $dccipr:ident,
        $lptimen:ident,
        $lptimrst:ident,
        $lptimsmen:ident,
        $lptimsel:ident,
        $lptim:ident,
        ($($in2sel:ident)?),
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl LowPowerTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::low_power;

            RCC {
                BUSENR {
                    $busenr Shared;
                    LPTIMEN { $lptimen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    LPTIMRST { $lptimrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    LPTIMSMEN { $lptimsmen }
                }
                DCCIPR {
                    $dccipr Shared;
                    LPTIMSEL { $lptimsel }
                }
            }

            LPTIM {
                $lptim;
                ISR {
                    ISR;
                    CMPM { CMPM }
                    ARRM { ARRM }
                    EXTTRIG { EXTTRIG }
                    CMPOK { CMPOK }
                    ARROK { ARROK }
                    UP { UP }
                    DOWN { DOWN }
                }
                ICR {
                    ICR;
                    CMPMCF { CMPMCF }
                    ARRMCF { ARRMCF }
                    EXTTRIGCF { EXTTRIGCF }
                    CMPOKCF { CMPOKCF }
                    ARROKCF { ARROKCF }
                    UPCF { UPCF }
                    DOWNCF { DOWNCF }
                }
                IER {
                    IER;
                    CMPMIE { CMPMIE }
                    ARRMIE { ARRMIE }
                    EXTTRIGIE { EXTTRIGIE }
                    CMPOKIE { CMPOKIE }
                    ARROKIE { ARROKIE }
                    UPIE { UPIE }
                    DOWNIE { DOWNIE }
                }
                CFGR {
                    CFGR;
                    CKSEL { CKSEL }
                    CKPOL { CKPOL }
                    CKFLT { CKFLT }
                    TRGFLT { TRGFLT }
                    PRESC { PRESC }
                    TRIGSEL { TRIGSEL }
                    TRIGEN { TRIGEN }
                    TIMOUT { TIMOUT }
                    WAVE { WAVE }
                    WAVPOL { WAVPOL }
                    PRELOAD { PRELOAD }
                    COUNTMODE { COUNTMODE }
                    ENC { ENC }
                }
                CR {
                    CR;
                    ENABLE { ENABLE }
                    SNGSTRT { SNGSTRT }
                    CNTSTRT { CNTSTRT }
                    COUNTRST { COUNTRST }
                    RSTARE { RSTARE }
                }
                CMP {
                    CMP;
                    CMP { CMP }
                }
                ARR {
                    ARR;
                    ARR { ARR }
                }
                CNT {
                    CNT;
                    CNT { CNT }
                }
                CFGR2 {
                    CFGR2;
                    IN1SEL { IN1SEL }
                    IN2SEL { $($in2sel Option)* }
                }
            }
        }
    };
}

map_low_power_tim! {
    "Extracts LPTIM1 register tokens.",
    periph_lptim1,
    "LPTIM1 peripheral variant.",
    Lptim1,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    LPTIM1EN,
    LPTIM1RST,
    LPTIM1LPEN,
    LPTIM1SEL,
    LPTIM1,
    (IN2SEL),
}

map_low_power_tim! {
    "Extracts LPTIM2 register tokens.",
    periph_lptim2,
    "LPTIM2 peripheral variant.",
    Lptim2,
    APB4ENR,
    APB4RSTR,
    APB4LPENR,
    D3CCIPR,
    LPTIM2EN,
    LPTIM2RST,
    LPTIM2LPEN,
    LPTIM2SEL,
    LPTIM2,
    (IN2SEL),
}

map_low_power_tim! {
    "Extracts LPTIM3 register tokens.",
    periph_lptim3,
    "LPTIM3 peripheral variant.",
    Lptim3,
    APB4ENR,
    APB4RSTR,
    APB4LPENR,
    D3CCIPR,
    LPTIM3EN,
    LPTIM3RST,
    LPTIM3LPEN,
    LPTIM345SEL,
    LPTIM3,
    (),
}

map_low_power_tim! {
    "Extracts LPTIM4 register tokens.",
    periph_lptim4,
    "LPTIM4 peripheral variant.",
    Lptim4,
    APB4ENR,
    APB4RSTR,
    APB4LPENR,
    D3CCIPR,
    LPTIM4EN,
    LPTIM4RST,
    LPTIM4LPEN,
    LPTIM345SEL,
    LPTIM4,
    (),
}

map_low_power_tim! {
    "Extracts LPTIM5 register tokens.",
    periph_lptim5,
    "LPTIM5 peripheral variant.",
    Lptim5,
    APB4ENR,
    APB4RSTR,
    APB4LPENR,
    D3CCIPR,
    LPTIM5EN,
    LPTIM5RST,
    LPTIM5LPEN,
    LPTIM345SEL,
    LPTIM5,
    (),
}
