//! Advanced-control timers.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic advanced-control timer peripheral variant.
    pub trait AdvancedTimMap {}

    /// Generic advanced-control timer peripheral.
    pub struct AdvancedTimPeriph;

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

    DBGMCU {
        APB2FZ1 {
            0x20 RwReg Shared;
            DBG_TIM { RwRwRegFieldBit }
        }
    }

    TIM {
        CR1 {
            0x20 RwReg;
            CEN { RwRwRegFieldBit }
            UDIS { RwRwRegFieldBit }
            URS { RwRwRegFieldBit }
            OPM { RwRwRegFieldBit }
            DIR { RwRwRegFieldBit }
            CMS { RwRwRegFieldBits }
            ARPE { RwRwRegFieldBit }
            CKD { RwRwRegFieldBits }
            UIFREMAP { RwRwRegFieldBit }
        }
        CR2 {
            0x20 RwReg;
            CCPC { RwRwRegFieldBit }
            CCUS { RwRwRegFieldBit }
            CCDS { RwRwRegFieldBit }
            MMS { RwRwRegFieldBits }
            TI1S { RwRwRegFieldBit }
            OIS1 { RwRwRegFieldBit }
            OIS1N { RwRwRegFieldBit }
            OIS2 { RwRwRegFieldBit }
            OIS2N { RwRwRegFieldBit }
            OIS3 { RwRwRegFieldBit }
            OIS3N { RwRwRegFieldBit }
            OIS4 { RwRwRegFieldBit }
            OIS5 { RwRwRegFieldBit }
            OIS6 { RwRwRegFieldBit }
            MMS2 { RwRwRegFieldBits }
        }
        SMCR {
            0x20 RwReg;
            SMS { RwRwRegFieldBits }
            TS { RwRwRegFieldBits }
            MSM { RwRwRegFieldBit }
            ETF { RwRwRegFieldBits }
            ETPS { RwRwRegFieldBits }
            ECE { RwRwRegFieldBit }
            ETP { RwRwRegFieldBit }
            SMS_3 { RwRwRegFieldBit }
            TS_4_3 { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwReg;
            UIE { RwRwRegFieldBit }
            CC1IE { RwRwRegFieldBit }
            CC2IE { RwRwRegFieldBit }
            CC3IE { RwRwRegFieldBit }
            COMIE { RwRwRegFieldBit }
            TIE { RwRwRegFieldBit }
            BIE { RwRwRegFieldBit }
            UDE { RwRwRegFieldBit }
            CC1DE { RwRwRegFieldBit }
            CC2DE { RwRwRegFieldBit }
            CC3DE { RwRwRegFieldBit }
            CC4DE { RwRwRegFieldBit }
            CC4IE { RwRwRegFieldBit }
            COMDE { RwRwRegFieldBit }
            TDE { RwRwRegFieldBit }
        }
        SR {
            0x20 RwReg;
            UIF { RwRwRegFieldBit }
            CC1IF { RwRwRegFieldBit }
            CC2IF { RwRwRegFieldBit }
            CC3IF { RwRwRegFieldBit }
            CC4IF { RwRwRegFieldBit }
            COMIF { RwRwRegFieldBit }
            BIF { RwRwRegFieldBit }
            B2IF { RwRwRegFieldBit }
            TIF { RwRwRegFieldBit }
            CC1OF { RwRwRegFieldBit }
            CC2OF { RwRwRegFieldBit }
            CC3OF { RwRwRegFieldBit }
            CC4OF { RwRwRegFieldBit }
            SBIF { RwRwRegFieldBit }
            CC5IF { RwRwRegFieldBit }
            CC6IF { RwRwRegFieldBit }
        }
        EGR {
            0x20 WoReg;
            UG { WoWoRegFieldBit }
            CC1G { WoWoRegFieldBit }
            CC2G { WoWoRegFieldBit }
            CC3G { WoWoRegFieldBit }
            CC4G { WoWoRegFieldBit }
            COMG { WoWoRegFieldBit }
            TG { WoWoRegFieldBit }
            BG { WoWoRegFieldBit }
            B2G { WoWoRegFieldBit }
        }
        CCMR1 {
            @Output 0x20 RwReg;
            CC1S { RwRwRegFieldBits }
            OC1FE { RwRwRegFieldBit }
            OC1PE { RwRwRegFieldBit }
            OC1M { RwRwRegFieldBits }
            OC1CE { RwRwRegFieldBit }
            CC2S { RwRwRegFieldBits }
            OC2FE { RwRwRegFieldBit }
            OC2PE { RwRwRegFieldBit }
            OC2M { RwRwRegFieldBits }
            OC2CE { RwRwRegFieldBit }
            OC1M_3 { RwRwRegFieldBit }
            OC2M_3 { RwRwRegFieldBit }
            @Input 0x20 RwReg;
            CC1S { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC1F { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits }
            IC2PSC { RwRwRegFieldBits }
            IC2F { RwRwRegFieldBits }
        }
        CCMR2 {
            @Output 0x20 RwReg;
            CC3S { RwRwRegFieldBits }
            OC3FE { RwRwRegFieldBit }
            OC3PE { RwRwRegFieldBit }
            OC3M { RwRwRegFieldBits }
            OC3CE { RwRwRegFieldBit }
            CC4S { RwRwRegFieldBits }
            OC4FE { RwRwRegFieldBit }
            OC4PE { RwRwRegFieldBit }
            OC4M { RwRwRegFieldBits }
            OC4CE { RwRwRegFieldBit }
            OC3M_3 { RwRwRegFieldBit }
            OC4M_3 { RwRwRegFieldBit }
            @Input 0x20 RwReg;
            CC3S { RwRwRegFieldBits }
            IC3PSC { RwRwRegFieldBits }
            IC3F { RwRwRegFieldBits }
            CC4S { RwRwRegFieldBits }
            IC4PSC { RwRwRegFieldBits }
            IC4F { RwRwRegFieldBits }
        }
        CCER {
            0x20 RwReg;
            CC1E { RwRwRegFieldBit }
            CC1P { RwRwRegFieldBit }
            CC1NE { RwRwRegFieldBit }
            CC1NP { RwRwRegFieldBit }
            CC2E { RwRwRegFieldBit }
            CC2P { RwRwRegFieldBit }
            CC2NE { RwRwRegFieldBit }
            CC2NP { RwRwRegFieldBit }
            CC3E { RwRwRegFieldBit }
            CC3P { RwRwRegFieldBit }
            CC3NE { RwRwRegFieldBit }
            CC3NP { RwRwRegFieldBit }
            CC4E { RwRwRegFieldBit }
            CC4P { RwRwRegFieldBit }
            CC4NP { RwRwRegFieldBit }
            CC5E { RwRwRegFieldBit }
            CC5P { RwRwRegFieldBit }
            CC6E { RwRwRegFieldBit }
            CC6P { RwRwRegFieldBit }
        }
        CNT {
            0x20 RwReg;
            CNT { RwRwRegFieldBits }
            UIFCPY { RoRwRegFieldBit }
        }
        PSC {
            0x20 RwReg;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwReg;
            ARR { RwRwRegFieldBits }
        }
        RCR {
            0x20 RwReg;
            REP { RwRwRegFieldBits }
        }
        CCR1 {
            0x20 RwReg;
            CCR1 { RwRwRegFieldBits }
        }
        CCR2 {
            0x20 RwReg;
            CCR2 { RwRwRegFieldBits }
        }
        CCR3 {
            0x20 RwReg;
            CCR3 { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwReg;
            CCR4 { RwRwRegFieldBits }
        }
        BDTR {
            0x20 RwReg;
            DTG { RwRwRegFieldBits }
            LOCK { RwRwRegFieldBits }
            OSSI { RwRwRegFieldBit }
            OSSR { RwRwRegFieldBit }
            BKE { RwRwRegFieldBit }
            BKP { RwRwRegFieldBit }
            AOE { RwRwRegFieldBit }
            MOE { RwRwRegFieldBit }
            BKF { RwRwRegFieldBits }
            BK2F { RwRwRegFieldBits }
            BK2E { RwRwRegFieldBit }
            BK2P { RwRwRegFieldBit }
        }
        DCR {
            0x20 RwReg;
            DBA { RwRwRegFieldBits }
            DBL { RwRwRegFieldBits }
        }
        DMAR {
            0x20 RwReg;
            DMAB { RwRwRegFieldBits }
        }
        CCMR3 {
            @Output 0x20 RwReg;
            OC5FE { RwRwRegFieldBit }
            OC5PE { RwRwRegFieldBit }
            OC5M { RwRwRegFieldBits }
            OC5CE { RwRwRegFieldBit }
            OC6FE { RwRwRegFieldBit }
            OC6PE { RwRwRegFieldBit }
            OC6M { RwRwRegFieldBits }
            OC6CE { RwRwRegFieldBit }
            OC5M_3 { RwRwRegFieldBit }
            OC6M_3 { RwRwRegFieldBit }
        }
        CCR5 {
            0x20 RwReg;
            CCR5 { RwRwRegFieldBits }
            GC5C1 { RwRwRegFieldBit }
            GC5C2 { RwRwRegFieldBit }
            GC5C3 { RwRwRegFieldBit }
        }
        CCR6 {
            0x20 RwReg;
            CCR6 { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_advanced_tim {
    (
        $tim_macro_doc:expr,
        $tim_macro:ident,
        $tim_ty_doc:expr,
        $tim_ty:ident,
        $timen:ident,
        $timrst:ident,
        $timsmen:ident,
        $timstop:ident,
        $tim:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl AdvancedTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::advanced;

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
                    TIMSMEN{ $timsmen }
                }
            }

            DBGMCU {
                DBGMCU;
                APB2FZ1 {
                    APB2FZ1 Shared;
                    DBG_TIM { $timstop }
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
                    DIR { DIR }
                    CMS { CMS }
                    ARPE { ARPE }
                    CKD { CKD }
                    UIFREMAP { UIFREMAP }
                }
                CR2 {
                    CR2;
                    CCPC { CCPC }
                    CCUS { CCUS }
                    CCDS { CCDS }
                    MMS { MMS }
                    TI1S { TI1S }
                    OIS1 { OIS1 }
                    OIS1N { OIS1N }
                    OIS2 { OIS2 }
                    OIS2N { OIS2N }
                    OIS3 { OIS3 }
                    OIS3N { OIS3N }
                    OIS4 { OIS4 }
                    OIS5 { OIS5 }
                    OIS6 { OIS6 }
                    MMS2 { MMS2 }
                }
                SMCR {
                    SMCR;
                    SMS { SMS }
                    TS { TS }
                    MSM { MSM }
                    ETF { ETF }
                    ETPS { ETPS }
                    ECE { ECE }
                    ETP { ETP }
                    SMS_3 { SMS_3 }
                    TS_4_3 { TS_4_3 }
                }
                DIER {
                    DIER;
                    UIE { UIE }
                    CC1IE { CC1IE }
                    CC2IE { CC2IE }
                    CC3IE { CC3IE }
                    COMIE { COMIE }
                    TIE { TIE }
                    BIE { BIE }
                    UDE { UDE }
                    CC1DE { CC1DE }
                    CC2DE { CC2DE }
                    CC3DE { CC3DE }
                    CC4DE { CC4DE }
                    CC4IE { CC4IE }
                    COMDE { COMDE }
                    TDE { TDE }
                }
                SR {
                    SR;
                    UIF { UIF }
                    CC1IF { CC1IF }
                    CC2IF { CC2IF }
                    CC3IF { CC3IF }
                    CC4IF { CC4IF }
                    COMIF { COMIF }
                    TIF { TIF }
                    BIF { BIF }
                    B2IF { B2IF }
                    CC1OF { CC1OF }
                    CC2OF { CC2OF }
                    CC3OF { CC3OF }
                    CC4OF { CC4OF }
                    SBIF { SBIF }
                    CC5IF { CC5IF }
                    CC6IF { CC6IF }
                }
                EGR {
                    EGR;
                    UG { UG }
                    CC1G { CC1G }
                    CC2G { CC2G }
                    CC3G { CC3G }
                    CC4G { CC4G }
                    COMG { COMG }
                    TG { TG }
                    BG { BG }
                    B2G { B2G }
                }
                CCMR1 {
                    @Output CCMR1_Output;
                    CC1S { CC1S }
                    OC1FE { OC1FE }
                    OC1PE { OC1PE }
                    OC1M { OC1M }
                    OC1CE { OC1CE }
                    CC2S { CC2S }
                    OC2FE { OC2FE }
                    OC2PE { OC2PE }
                    OC2M { OC2M }
                    OC2CE { OC2CE }
                    OC1M_3 { OC1M_3 }
                    OC2M_3 { OC2M_3 }
                    @Input CCMR1_Input;
                    CC1S { CC1S }
                    IC1PSC { IC1PSC }
                    IC1F { IC1F }
                    CC2S { CC2S }
                    IC2PSC { IC2PSC }
                    IC2F { IC2F }
                }
                CCMR2 {
                    @Output CCMR2_Output;
                    CC3S { CC3S }
                    OC3FE { OC3FE }
                    OC3PE { OC3PE }
                    OC3M { OC3M }
                    OC3CE { OC3CE }
                    CC4S { CC4S }
                    OC4FE { OC4FE }
                    OC4PE { OC4PE }
                    OC4M { OC4M }
                    OC4CE { OC4CE }
                    OC3M_3 { OC3M_3 }
                    OC4M_3 { OC4M_3 }
                    @Input CCMR2_Input;
                    CC3S { CC3S }
                    IC3PSC { IC3PSC }
                    IC3F { IC3F }
                    CC4S { CC4S }
                    IC4PSC { IC4PSC }
                    IC4F { IC4F }
                }
                CCER {
                    CCER;
                    CC1E { CC1E }
                    CC1P { CC1P }
                    CC1NE { CC1NE }
                    CC1NP { CC1NP }
                    CC2E { CC2E }
                    CC2P { CC2P }
                    CC2NE { CC2NE }
                    CC2NP { CC2NP }
                    CC3E { CC3E }
                    CC3P { CC3P }
                    CC3NE { CC3NE }
                    CC3NP { CC3NP }
                    CC4E { CC4E }
                    CC4P { CC4P }
                    CC4NP { CC4NP }
                    CC5E { CC5E }
                    CC5P { CC5P }
                    CC6E { CC6E }
                    CC6P { CC6P }
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
                RCR {
                    RCR;
                    REP { REP }
                }
                CCR1 {
                    CCR1;
                    CCR1 { CCR1 }
                }
                CCR2 {
                    CCR2;
                    CCR2 { CCR2 }
                }
                CCR3 {
                    CCR3;
                    CCR3 { CCR3 }
                }
                CCR4 {
                    CCR4;
                    CCR4 { CCR4 }
                }
                BDTR {
                    BDTR;
                    DTG { DTG }
                    LOCK { LOCK }
                    OSSI { OSSI }
                    OSSR { OSSR }
                    BKE { BKE }
                    BKP { BKP }
                    AOE { AOE }
                    MOE { MOE }
                    BKF { BKF }
                    BK2F { BK2F }
                    BK2E { BK2E }
                    BK2P { BK2P }
                }
                DCR {
                    DCR;
                    DBA { DBA }
                    DBL { DBL }
                }
                DMAR {
                    DMAR;
                    DMAB { DMAB }
                }
                CCMR3 {
                    @Output CCMR3_Output;
                    OC5FE { OC5FE }
                    OC5PE { OC5PE }
                    OC5M { OC5M }
                    OC5CE { OC5CE }
                    OC6FE { OC6FE }
                    OC6PE { OC6PE }
                    OC6M { OC6M }
                    OC6CE { OC6CE }
                    OC5M_3 { OC5M_3 }
                    OC6M_3 { OC6M_3 }
                }
                CCR5 {
                    CCR5;
                    CCR5 { CCR5 }
                    GC5C1 { GC5C1 }
                    GC5C2 { GC5C2 }
                    GC5C3 { GC5C3 }
                }
                CCR6 {
                    CCR6;
                    CCR6 { CCR6 }
                }
            }
        }
    };
}

map_advanced_tim! {
    "Extracts TIM1 register tokens.",
    periph_tim1,
    "TIM1 peripheral variant",
    Tim1,
    TIM1EN,
    TIM1RST,
    TIM1LPEN,
    DBG_TIM1,
    TIM1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
}

map_advanced_tim! {
    "Extracts TIM8 register tokens.",
    periph_tim8,
    "TIM8 peripheral variant",
    Tim8,
    TIM8EN,
    TIM8RST,
    TIM8LPEN,
    DBG_TIM8,
    TIM8,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
}
