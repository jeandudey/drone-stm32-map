//! General-purpose timers.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic general-purpose timer peripheral variant.
    pub trait GeneralTimMap {}

    /// Generic general-purpose timer peripheral.
    pub struct GeneralTimPeriph;

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
            DIR { RwRwRegFieldBit Option }
            CMS { RwRwRegFieldBits Option }
            ARPE { RwRwRegFieldBit }
            CKD { RwRwRegFieldBits }
            UIFREMAP { RwRwRegFieldBit }
        }
        CR2 {
            0x20 RwReg Option;
            CCPC { RwRwRegFieldBit Option }
            CCUS { RwRwRegFieldBit Option }
            CCDS { RwRwRegFieldBit Option }
            MMS { RwRwRegFieldBits Option }
            TI1S { RwRwRegFieldBit Option }
            OIS1 { RwRwRegFieldBit Option }
            OIS1N { RwRwRegFieldBit Option }
            OIS2 { RwRwRegFieldBit Option }
        }
        SMCR {
            0x20 RwReg Option;
            SMS { RwRwRegFieldBits }
            TS { RwRwRegFieldBits }
            MSM { RwRwRegFieldBit }
            ETF { RwRwRegFieldBits Option }
            ETPS { RwRwRegFieldBits Option }
            ECE { RwRwRegFieldBit Option }
            ETP { RwRwRegFieldBit Option }
            SMS_3 { RwRwRegFieldBit }
            TS_4_3 { RwRwRegFieldBits }
        }
        DIER {
            0x20 RwReg;
            UIE { RwRwRegFieldBit }
            CC1IE { RwRwRegFieldBit }
            CC2IE { RwRwRegFieldBit Option }
            CC3IE { RwRwRegFieldBit Option }
            CC4IE { RwRwRegFieldBit Option }
            COMIE { RwRwRegFieldBit Option }
            TIE { RwRwRegFieldBit Option }
            BIE { RwRwRegFieldBit Option }
            UDE { RwRwRegFieldBit Option }
            CC1DE { RwRwRegFieldBit Option }
            CC2DE { RwRwRegFieldBit Option }
            CC3DE { RwRwRegFieldBit Option }
            CC4DE { RwRwRegFieldBit Option }
            COMDE { RwRwRegFieldBit Option }
            TDE { RwRwRegFieldBit Option }
        }
        SR {
            0x20 RwReg;
            UIF { RwRwRegFieldBit }
            CC1IF { RwRwRegFieldBit }
            CC2IF { RwRwRegFieldBit Option }
            CC3IF { RwRwRegFieldBit Option }
            CC4IF { RwRwRegFieldBit Option }
            COMIF { RwRwRegFieldBit Option }
            TIF { RwRwRegFieldBit Option }
            BIF { RwRwRegFieldBit Option }
            CC1OF { RwRwRegFieldBit }
            CC2OF { RwRwRegFieldBit Option }
            CC3OF { RwRwRegFieldBit Option }
            CC4OF { RwRwRegFieldBit Option }
        }
        EGR {
            0x20 WoReg;
            UG { WoWoRegFieldBit }
            CC1G { WoWoRegFieldBit }
            CC2G { WoWoRegFieldBit Option }
            CC3G { WoWoRegFieldBit Option }
            CC4G { WoWoRegFieldBit Option }
            COMG { WoWoRegFieldBit Option }
            TG { WoWoRegFieldBit Option }
            BG { WoWoRegFieldBit Option }
        }
        CCMR1 {
            @Output 0x20 RwReg;
            CC1S { RwRwRegFieldBits }
            OC1FE { RwRwRegFieldBit }
            OC1PE { RwRwRegFieldBit }
            OC1M { RwRwRegFieldBits }
            OC1CE { RwRwRegFieldBit Option }
            CC2S { RwRwRegFieldBits Option }
            OC2FE { RwRwRegFieldBit Option }
            OC2PE { RwRwRegFieldBit Option }
            OC2M { RwRwRegFieldBits Option }
            OC2CE { RwRwRegFieldBit Option }
            OC1M_3 { RwRwRegFieldBit }
            OC2M_3 { RwRwRegFieldBit Option }
            @Input 0x20 RwReg;
            CC1S { RwRwRegFieldBits }
            IC1PSC { RwRwRegFieldBits }
            IC1F { RwRwRegFieldBits }
            CC2S { RwRwRegFieldBits Option }
            IC2PSC { RwRwRegFieldBits Option }
            IC2F { RwRwRegFieldBits Option }
        }
        CCMR2 {
            @Output 0x20 RwReg Option;
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
            @Input 0x20 RwReg Option;
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
            CC1NP { RwRwRegFieldBit }
            CC2E { RwRwRegFieldBit Option }
            CC2P { RwRwRegFieldBit Option }
            CC2NP { RwRwRegFieldBit Option }
            CC1NE { RwRwRegFieldBit Option }
            CC3E { RwRwRegFieldBit Option }
            CC3P { RwRwRegFieldBit Option }
            CC3NP { RwRwRegFieldBit Option }
            CC4E { RwRwRegFieldBit Option }
            CC4P { RwRwRegFieldBit Option }
            CC4NP { RwRwRegFieldBit Option }
        }
        CNT {
            0x20 RwReg;
            UIFCPY_CNT31 { RwRwRegFieldBit Option }
            UIFCPY { RoRwRegFieldBit Option }
            CNT { RwRwRegFieldBits }
        }
        PSC {
            0x20 RwReg;
            PSC { RwRwRegFieldBits }
        }
        ARR {
            0x20 RwReg Option;
            ARR { RwRwRegFieldBits }
        }
        RCR {
            0x20 RwReg Option;
            REP { RwRwRegFieldBits }
        }
        CCR1 {
            0x20 RwReg;
            CCR1 { RwRwRegFieldBits }
        }
        CCR2 {
            0x20 RwReg Option;
            CCR2 { RwRwRegFieldBits }
        }
        CCR3 {
            0x20 RwReg Option;
            CCR3 { RwRwRegFieldBits }
        }
        CCR4 {
            0x20 RwReg Option;
            CCR4 { RwRwRegFieldBits }
        }
        BDTR {
            0x20 RwReg Option;
            DTG { RwRwRegFieldBits }
            LOCK { RwRwRegFieldBits }
            OSSI { RwRwRegFieldBit }
            OSSR { RwRwRegFieldBit }
            BKE { RwRwRegFieldBit }
            BKP { RwRwRegFieldBit }
            AOE { RwRwRegFieldBit }
            MOE { RwRwRegFieldBit }
            BKF { RwRwRegFieldBits }
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
        AF1 {
            0x20 RwReg Option;
            BKINE { RwRwRegFieldBit Option }
            BKCMP1E { RwRwRegFieldBit Option }
            BKCMP2E { RwRwRegFieldBit Option }
            BKDF1BKNE { RwRwRegFieldBit Option }
            BKINP { RwRwRegFieldBit Option }
            BKCMP1P { RwRwRegFieldBit Option }
            BKCMP2P { RwRwRegFieldBit Option }
            ETRSEL { RwRwRegFieldBits Option }
        }
        TISEL {
            0x20 RwReg Option;
            TI1SEL { RwRwRegFieldBits }
            TI2SEL { RwRwRegFieldBits Option }
            TI3SEL { RwRwRegFieldBits Option }
            TI4SEL { RwRwRegFieldBits Option }
        }
    }
}

macro_rules! map_general_tim {
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
        ($($dir:ident)?, $($cms:ident)?),
        ($(
            $cr2:ident,
            $($ccpc:ident)?,
            $($ccus:ident)?,
            $($ccds:ident)?,
            $($mms:ident)?,
            $($ti1s:ident)?,
            $($ois1:ident)?,
            $($ois1n:ident)?,
            $($ois2:ident)?
        )?),
        ($(
            $smcr:ident,
            $($etf:ident)?,
            $($etps:ident)?,
            $($ece:ident)?,
            $($etp:ident)?
        )?),
        (
            $($cc2ie:ident)?,
            $($cc3ie:ident)?,
            $($cc4ie:ident)?,
            $($comie:ident)?,
            $($tie:ident)?,
            $($bie:ident)?,
            $($ude:ident)?,
            $($cc1de:ident)?,
            $($cc2de:ident)?,
            $($cc3de:ident)?,
            $($cc4de:ident)?,
            $($comde:ident)?,
            $($tde:ident)?
        ),
        (
            $($cc2if:ident)?,
            $($cc3if:ident)?,
            $($cc4if:ident)?,
            $($comif:ident)?,
            $($tif:ident)?,
            $($bif:ident)?,
            $($cc2of:ident)?,
            $($cc3of:ident)?,
            $($cc4of:ident)?
        ),
        (
            $($cc2g:ident)?,
            $($cc3g:ident)?,
            $($cc4g:ident)?,
            $($comg:ident)?,
            $($tg:ident)?,
            $($bg:ident)?
        ),
        (
            $($oc1ce:ident)?,
            $($cc2s:ident)?,
            $($oc2fe:ident)?,
            $($oc2pe:ident)?,
            $($oc2m:ident)?,
            $($oc2ce:ident)?,
            $($oc2m_3:ident)?,
            $($ic2psc:ident)?,
            $($ic2f:ident)?
        ),
        ($($ccmr2_output:ident)?, $($ccmr2_input:ident)?),
        (
            $($cc2e:ident)?,
            $($cc2p:ident)?,
            $($cc2np:ident)?,
            $($cc1ne:ident)?,
            $($cc3e:ident)?,
            $($cc3p:ident)?,
            $($cc3np:ident)?,
            $($cc4e:ident)?,
            $($cc4p:ident)?,
            $($cc4np:ident)?
        ),
        ($($uifcpy_cnt31:ident)?, $($uifcpy:ident)?),
        ($($arr:ident)?),
        ($($rcr:ident)?),
        (
            $($ccr2:ident)?,
            $($ccr3:ident)?,
            $($ccr4:ident)?
        ),
        ($($bdtr:ident)?),
        ($(
            $af1:ident,
            $($bkine:ident)?,
            $($bkcmp1e:ident)?,
            $($bkcmp2e:ident)?,
            $($bkdf1bkne:ident)?,
            $($bkinp:ident)?,
            $($bkcmp1p:ident)?,
            $($bkcmp2p:ident)?,
            $($etrsel:ident)?
        )?),
        ($(
            $tisel:ident,
            $($ti2sel:ident)?,
            $($ti3sel:ident)?,
            $($ti4sel:ident)?
        )?),
    ) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl GeneralTimMap for $tim_ty {}

            drone_stm32_map_pieces::reg;
            crate::general;

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
                    DIR { $($dir Option)* }
                    CMS { $($cms Option)* }
                    ARPE { ARPE }
                    CKD { CKD }
                    UIFREMAP { UIFREMAP }
                }
                CR2 {
                    $(
                        $cr2 Option;
                        CCPC { $($ccpc Option)* }
                        CCUS { $($ccus Option)* }
                        CCDS { $($ccds Option)* }
                        MMS { $($mms Option)* }
                        TI1S { $($ti1s Option)* }
                        OIS1 { $($ois1 Option)* }
                        OIS1N { $($ois1n Option)* }
                        OIS2 { $($ois2 Option)* }
                    )*
                }
                SMCR {
                    $(
                        $smcr Option;
                        SMS { SMS }
                        TS { TS }
                        MSM { MSM }
                        ETF { $($etf Option)* }
                        ETPS { $($etps Option)* }
                        ECE { $($ece Option)* }
                        ETP { $($etp Option)* }
                        SMS_3 { SMS_3 }
                        TS_4_3 { TS_4_3 }
                    )*
                }
                DIER {
                    DIER;
                    UIE { UIE }
                    CC1IE { CC1IE }
                    CC2IE { $($cc2ie Option)* }
                    CC3IE { $($cc3ie Option)* }
                    CC4IE { $($cc4ie Option)* }
                    COMIE { $($comie Option)* }
                    TIE { $($tie Option)* }
                    BIE { $($bie Option)* }
                    UDE { $($ude Option)* }
                    CC1DE { $($cc1de Option)* }
                    CC2DE { $($cc2de Option)* }
                    CC3DE { $($cc3de Option)* }
                    CC4DE { $($cc4de Option)* }
                    COMDE { $($comde Option)* }
                    TDE { $($tde Option)* }
                }
                SR {
                    SR;
                    UIF { UIF }
                    CC1IF { CC1IF }
                    CC2IF { $($cc2if Option)* }
                    CC3IF { $($cc3if Option)* }
                    CC4IF { $($cc4if Option)* }
                    COMIF { $($comif Option)* }
                    TIF { $($tif Option)* }
                    BIF { $($bif Option)* }
                    CC1OF { CC1OF }
                    CC2OF { $($cc2of Option)* }
                    CC3OF { $($cc3of Option)* }
                    CC4OF { $($cc4of Option)* }
                }
                EGR {
                    EGR;
                    UG { UG }
                    CC1G { CC1G }
                    CC2G { $($cc2g Option)* }
                    CC3G { $($cc3g Option)* }
                    CC4G { $($cc4g Option)* }
                    COMG { $($comg Option)* }
                    TG { $($tg Option)* }
                    BG { $($bg Option)* }
                }
                CCMR1 {
                    @Output CCMR1_Output;
                    CC1S { CC1S }
                    OC1FE { OC1FE }
                    OC1PE { OC1PE }
                    OC1M { OC1M }
                    OC1CE { $($oc1ce Option)* }
                    CC2S { $($cc2s Option)* }
                    OC2FE { $($oc2fe Option)* }
                    OC2PE { $($oc2pe Option)* }
                    OC2M { $($oc2m Option)* }
                    OC2CE { $($oc2ce Option)* }
                    OC1M_3 { OC1M_3 }
                    OC2M_3 { $($oc2m_3 Option)* }
                    @Input CCMR1_Input;
                    CC1S { CC1S }
                    IC1PSC { IC1PSC }
                    IC1F { IC1F }
                    CC2S { $($cc2s Option)* }
                    IC2PSC { $($ic2psc Option)* }
                    IC2F { $($ic2f Option)* }
                }
                CCMR2 {
                    @Output $(
                        $ccmr2_output Option;
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
                    )*
                    @Input $(
                        $ccmr2_input Option;
                        CC3S { CC3S }
                        IC3PSC { IC3PSC }
                        IC3F { IC3F }
                        CC4S { CC4S }
                        IC4PSC { IC4PSC }
                        IC4F { IC4F }
                    )*
                }
                CCER {
                    CCER;
                    CC1E { CC1E }
                    CC1P { CC1P }
                    CC1NP { CC1NP }
                    CC2E { $($cc2e Option)* }
                    CC2P { $($cc2p Option)* }
                    CC2NP { $($cc2np Option)* }
                    CC1NE { $($cc1ne Option)* }
                    CC3E { $($cc3e Option)* }
                    CC3P { $($cc3p Option)* }
                    CC3NP { $($cc3np Option)* }
                    CC4E { $($cc4e Option)* }
                    CC4P { $($cc4p Option)* }
                    CC4NP { $($cc4np Option)* }
                }
                CNT {
                    CNT;
                    UIFCPY_CNT31 { $($uifcpy_cnt31 Option)* }
                    UIFCPY { $($uifcpy Option)* }
                    CNT { CNT }
                }
                PSC {
                    PSC;
                    PSC { PSC }
                }
                ARR {
                    $(
                        $arr Option;
                        ARR { ARR }
                    )*
                }
                RCR {
                    $(
                        $rcr Option;
                        REP { REP }
                    )*
                }
                CCR1 {
                    CCR1;
                    CCR1 { CCR1 }
                }
                CCR2 {
                    $(
                        $ccr2 Option;
                        CCR2 { CCR2 }
                    )*
                }
                CCR3 {
                    $(
                        $ccr3 Option;
                        CCR3 { CCR3 }
                    )*
                }
                CCR4 {
                    $(
                        $ccr4 Option;
                        CCR4 { CCR4 }
                    )*
                }
                BDTR {
                    $(
                        $bdtr Option;
                        DTG { DTG }
                        LOCK { LOCK }
                        OSSI { OSSI }
                        OSSR { OSSR }
                        BKE { BKE }
                        BKP { BKP }
                        AOE { AOE }
                        MOE { MOE }
                        BKF { BKF }
                    )*
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
                AF1 {
                    $(
                        $af1 Option;
                        BKINE { $($bkine Option)* }
                        BKCMP1E { $($bkcmp1e Option)* }
                        BKCMP2E { $($bkcmp2e Option)* }
                        BKDF1BKNE { $($bkdf1bkne Option)* }
                        BKINP { $($bkinp Option)* }
                        BKCMP1P { $($bkcmp1p Option)* }
                        BKCMP2P { $($bkcmp2p Option)* }
                        ETRSEL { $($etrsel Option)* }
                    )*
                }
                TISEL {
                    $(
                        $tisel Option;
                        TI1SEL { TI1SEL }
                        TI2SEL { $($ti2sel Option)* }
                        TI3SEL { $($ti3sel Option)* }
                        TI4SEL { $($ti4sel Option)* }
                    )*
                }
            }
        }
    };
}

map_general_tim! {
    "Extracts TIM2 register tokens.",
    periph_tim2,
    "TIM2 peripheral variant.",
    Tim2,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM2EN,
    TIM2RST,
    TIM2LPEN,
    TIM2,
    (DIR, CMS),
    (CR2,,, CCDS, MMS, TI1S,,,),
    (SMCR, ETF, ETPS, ECE, ETP),
    (CC2IE, CC3IE, CC4IE,, TIE,, UDE, CC1DE, CC2DE, CC3DE, CC4DE,, TDE),
    (CC2IF, CC3IF, CC4IF,, TIF,, CC2OF, CC3OF, CC4OF),
    (CC2G, CC3G, CC4G,, TG,),
    (OC1CE, CC2S, OC2FE, OC2PE, OC2M, OC2CE, OC2M_3, IC2PSC, IC2F),
    (CCMR2_Output, CCMR2_Input),
    (CC2E, CC2P, CC2NP,, CC3E, CC3P, CC3NP, CC4E, CC4P, CC4NP),
    (,),
    (ARR),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (AF1,,,,,,,, ETRSEL),
    (TISEL, TI2SEL, TI3SEL, TI4SEL),
}

map_general_tim! {
    "Extracts TIM3 register tokens.",
    periph_tim3,
    "TIM3 peripheral variant.",
    Tim3,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM3EN,
    TIM3RST,
    TIM3LPEN,
    TIM3,
    (DIR, CMS),
    (CR2,,, CCDS, MMS, TI1S,,,),
    (SMCR, ETF, ETPS, ECE, ETP),
    (CC2IE, CC3IE, CC4IE,, TIE,, UDE, CC1DE, CC2DE, CC3DE, CC4DE,, TDE),
    (CC2IF, CC3IF, CC4IF,, TIF,, CC2OF, CC3OF, CC4OF),
    (CC2G, CC3G, CC4G,, TG,),
    (OC1CE, CC2S, OC2FE, OC2PE, OC2M, OC2CE, OC2M_3, IC2PSC, IC2F),
    (CCMR2_Output, CCMR2_Input),
    (CC2E, CC2P, CC2NP,, CC3E, CC3P, CC3NP, CC4E, CC4P, CC4NP),
    (,),
    (ARR),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (AF1,,,,,,,, ETRSEL),
    (TISEL, TI2SEL, TI3SEL, TI4SEL),
}

map_general_tim! {
    "Extracts TIM4 register tokens.",
    periph_tim4,
    "TIM4 peripheral variant.",
    Tim4,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM4EN,
    TIM4RST,
    TIM4LPEN,
    TIM4,
    (DIR, CMS),
    (CR2,,, CCDS, MMS, TI1S,,,),
    (SMCR, ETF, ETPS, ECE, ETP),
    (CC2IE, CC3IE, CC4IE,, TIE,, UDE, CC1DE, CC2DE, CC3DE, CC4DE,, TDE),
    (CC2IF, CC3IF, CC4IF,, TIF,, CC2OF, CC3OF, CC4OF),
    (CC2G, CC3G, CC4G,, TG,),
    (OC1CE, CC2S, OC2FE, OC2PE, OC2M, OC2CE, OC2M_3, IC2PSC, IC2F),
    (CCMR2_Output, CCMR2_Input),
    (CC2E, CC2P, CC2NP,, CC3E, CC3P, CC3NP, CC4E, CC4P, CC4NP),
    (,),
    (ARR),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (AF1,,,,,,,, ETRSEL),
    (TISEL, TI2SEL, TI3SEL, TI4SEL),
}

map_general_tim! {
    "Extracts TIM5 register tokens.",
    periph_tim5,
    "TIM5 peripheral variant.",
    Tim5,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM5EN,
    TIM5RST,
    TIM5LPEN,
    TIM5,
    (DIR, CMS),
    (CR2,,, CCDS, MMS, TI1S,,,),
    (SMCR, ETF, ETPS, ECE, ETP),
    (CC2IE, CC3IE, CC4IE,, TIE,, UDE, CC1DE, CC2DE, CC3DE, CC4DE,, TDE),
    (CC2IF, CC3IF, CC4IF,, TIF,, CC2OF, CC3OF, CC4OF),
    (CC2G, CC3G, CC4G,, TG,),
    (OC1CE, CC2S, OC2FE, OC2PE, OC2M, OC2CE, OC2M_3, IC2PSC, IC2F),
    (CCMR2_Output, CCMR2_Input),
    (CC2E, CC2P, CC2NP,, CC3E, CC3P, CC3NP, CC4E, CC4P, CC4NP),
    (,),
    (ARR),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (AF1,,,,,,,, ETRSEL),
    (TISEL, TI2SEL, TI3SEL, TI4SEL),
}

map_general_tim! {
    "Extracts TIM12 register tokens.",
    periph_tim12,
    "TIM12 peripheral variant.",
    Tim12,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM12EN,
    TIM12RST,
    TIM12LPEN,
    TIM12,
    (,),
    (CR2,,,, MMS, TI1S,,,),
    (SMCR,,,,),
    (CC2IE,,,, TIE,,,,,,,,),
    (,,,, TIF,,CC2OF,,),
    (CC2G,,,, TG,),
    (,,,,,,,,),
    (,),
    (CC2E, CC2P, CC2NP,,,,,,,),
    (,),
    (ARR),
    (),
    (CCR2,,),
    (),
    (),
    (TISEL, TI2SEL,,),
}

map_general_tim! {
    "Extracts TIM13 register tokens.",
    periph_tim13,
    "TIM13 peripheral variant.",
    Tim13,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM13EN,
    TIM13RST,
    TIM13LPEN,
    TIM13,
    (,),
    (),
    (),
    (,,,,,,,,,,,,),
    (,,,, TIF,, CC2OF,,),
    (CC2G,,,,,),
    (,,,,,,,,),
    (,),
    (,,,,,,,,,),
    (,),
    (ARR),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (AF1,,,,,,,, ETRSEL),
    (TISEL,,,),
}

map_general_tim! {
    "Extracts TIM14 register tokens.",
    periph_tim14,
    "TIM14 peripheral variant.",
    Tim14,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    TIM14EN,
    TIM14RST,
    TIM14LPEN,
    TIM14,
    (,),
    (),
    (),
    (,,,,,,,,,,,,),
    (,,,, TIF,, CC2OF,,),
    (CC2G,,,,,),
    (,,,,,,,,),
    (,),
    (,,,,,,,,,),
    (,),
    (ARR),
    (),
    (CCR2, CCR3, CCR4),
    (),
    (AF1,,,,,,,, ETRSEL),
    (TISEL,,,),
}

map_general_tim! {
    "Extracts TIM15 register tokens.",
    periph_tim15,
    "TIM15 peripheral variant.",
    Tim15,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM15EN,
    TIM15RST,
    TIM15LPEN,
    TIM15,
    (,),
    (CR2, CCPC, CCUS, CCDS, MMS, TI1S, OIS1, OIS1N, OIS2),
    (SMCR,,,,),
    (CC2IE,,, COMIE, TIE, BIE, UDE, CC1DE, CC2DE,,, COMDE, TDE),
    (CC2IF,,, COMIF, TIF, BIF, CC2OF,,),
    (CC2G,,, COMG, TG, BG),
    (, CC2S, OC2FE, OC2PE, OC2M,, OC2M_3, IC2PSC, IC2F),
    (,),
    (CC2E, CC2P, CC2NP, CC1NE,,,,,,),
    (,UIFCPY),
    (ARR),
    (RCR),
    (CCR2,,),
    (BDTR),
    (AF1, BKINE, BKCMP1E, BKCMP2E, BKDF1BK0E, BKINP, BKCMP1P, BKCMP2P,),
    (TISEL, TI2SEL,,),
}

map_general_tim! {
    "Extracts TIM16 register tokens.",
    periph_tim16,
    "TIM15 peripheral variant.",
    Tim16,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM16EN,
    TIM16RST,
    TIM16LPEN,
    TIM16,
    (,),
    (CR2, CCPC, CCUS, CCDS,,, OIS1, OIS1N,),
    (),
    (,,, COMIE,, BIE, UDE, CC1DE,,,,,),
    (,,, COMIF,, BIF,,,),
    (,,, COMG,, BG),
    (,,,,,,,,),
    (,),
    (,,, CC1NE,,,,,,),
    (,UIFCPY),
    (ARR),
    (RCR),
    (,,),
    (BDTR),
    (AF1, BKINE, BKCMP1E, BKCMP2E, BKDF1BK1E, BKINP, BKCMP1P, BKCMP2P,),
    (TISEL,,,),
}

map_general_tim! {
    "Extracts TIM17 register tokens.",
    periph_tim17,
    "TIM17 peripheral variant.",
    Tim17,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    TIM17EN,
    TIM17RST,
    TIM17LPEN,
    TIM17,
    (,),
    (CR2, CCPC, CCUS, CCDS,,, OIS1, OIS1N,),
    (),
    (,,, COMIE,, BIE, UDE, CC1DE,,,,,),
    (,,, COMIF,, BIF,,,),
    (,,, COMG,, BG),
    (,,,,,,,,),
    (,),
    (,,, CC1NE,,,,,,),
    (,UIFCPY),
    (ARR),
    (RCR),
    (,,),
    (BDTR),
    (AF1, BKINE, BKCMP1E, BKCMP2E, BKDF1BK2E, BKINP, BKCMP1P, BKCMP2P,),
    (TISEL,,,),
}
