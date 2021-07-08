//! Analog-to-digital converters registers
//!
//! For STM32H7 Series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic ADC peripheral variant.
    pub trait AdcMap {}

    /// Generic ADC peripheral.
    pub struct AdcPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            ADCEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            ADCRST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            ADCSMEN { RwRwRegFieldBit }
        }
        DCCIPR {
            0x20 RwReg Shared;
            ADCSEL { RwRwRegFieldBits }
        }
    }

    ADC {
        ISR {
            0x20 RwReg;
            ADRDY { RwRwRegFieldBit }
            EOSMP { RwRwRegFieldBit }
            EOC { RwRwRegFieldBit }
            EOS { RwRwRegFieldBit }
            OVR { RwRwRegFieldBit }
            JEOC { RwRwRegFieldBit }
            JEOS { RwRwRegFieldBit }
            AWD1 { RwRwRegFieldBit }
            AWD2 { RwRwRegFieldBit }
            AWD3 { RwRwRegFieldBit }
            JQOVF { RwRwRegFieldBit }
            LDORDY { RoRwRegFieldBit }
        }
        IER {
            0x20 RwReg;
            ADRDYIE { RwRwRegFieldBit }
            EOSMPIE { RwRwRegFieldBit }
            EOCIE { RwRwRegFieldBit }
            EOSIE { RwRwRegFieldBit }
            OVRIE { RwRwRegFieldBit }
            JEOCIE { RwRwRegFieldBit }
            JEOSIE { RwRwRegFieldBit }
            AWD1IE { RwRwRegFieldBit }
            AWD2IE { RwRwRegFieldBit }
            AWD3IE { RwRwRegFieldBit }
            JQOVFIE { RwRwRegFieldBit }
        }
        CR {
            0x20 RwReg;
            ADEN { RwRwRegFieldBit }
            ADDIS { RwRwRegFieldBit }
            ADSTART { RwRwRegFieldBit }
            JADSTART { RwRwRegFieldBit }
            ADSTP { RwRwRegFieldBit }
            JADSTP { RwRwRegFieldBit }
            BOOST { RwRwRegFieldBit }
            ADCALLIN { RwRwRegFieldBit }
            LINCALRDYW1 { RwRwRegFieldBit }
            LINCALRDYW2 { RwRwRegFieldBit }
            LINCALRDYW3 { RwRwRegFieldBit }
            LINCALRDYW4 { RwRwRegFieldBit }
            LINCALRDYW5 { RwRwRegFieldBit }
            LINCALRDYW6 { RwRwRegFieldBit }
            ADVREGEN { RwRwRegFieldBit }
            DEEPPWD { RwRwRegFieldBit }
            ADCALDIF { RwRwRegFieldBit }
            ADCAL { RwRwRegFieldBit }
        }
        CFGR {
            0x20 RwReg;
            DMNGT { RwRwRegFieldBits }
            RES { RwRwRegFieldBits }
            EXTSEL { RwRwRegFieldBits }
            EXTEN { RwRwRegFieldBits }
            OVRMOD { RwRwRegFieldBit }
            CONT { RwRwRegFieldBit }
            AUTDLY { RwRwRegFieldBit }
            DISCEN { RwRwRegFieldBit }
            DISCNUM { RwRwRegFieldBits }
            JDISCEN { RwRwRegFieldBit }
            JQM { RwRwRegFieldBit }
            AWD1SGL { RwRwRegFieldBit }
            AWD1EN { RwRwRegFieldBit }
            JAWD1EN { RwRwRegFieldBit }
            JAUTO { RwRwRegFieldBit }
            AWDCH1CH { RwRwRegFieldBits }
            JQDIS { RwRwRegFieldBit }
        }
        CFGR2 {
            0x20 RwReg;
            JOVSE { RwRwRegFieldBit }
            ROVSE { RwRwRegFieldBit }
            OVSS { RwRwRegFieldBits }
            TROVS { RwRwRegFieldBit }
            ROVSM { RwRwRegFieldBit }
            RSHIFT1 { RwRwRegFieldBit }
            RSHIFT2 { RwRwRegFieldBit }
            RSHIFT3 { RwRwRegFieldBit }
            RSHIFT4 { RwRwRegFieldBit }
            OSVR { RwRwRegFieldBits }
            LSHIFT { RwRwRegFieldBits }
        }
        SMPR1 {
            0x20 RwReg;
            SMP0 { RwRwRegFieldBits }
            SMP1 { RwRwRegFieldBits }
            SMP2 { RwRwRegFieldBits }
            SMP3 { RwRwRegFieldBits }
            SMP4 { RwRwRegFieldBits }
            SMP5 { RwRwRegFieldBits }
            SMP6 { RwRwRegFieldBits }
            SMP7 { RwRwRegFieldBits }
            SMP8 { RwRwRegFieldBits }
            SMP9 { RwRwRegFieldBits }
        }
        SMPR2 {
            0x20 RwReg;
            SMP10 { RwRwRegFieldBits }
            SMP11 { RwRwRegFieldBits }
            SMP12 { RwRwRegFieldBits }
            SMP13 { RwRwRegFieldBits }
            SMP14 { RwRwRegFieldBits }
            SMP15 { RwRwRegFieldBits }
            SMP16 { RwRwRegFieldBits }
            SMP17 { RwRwRegFieldBits }
            SMP18 { RwRwRegFieldBits }
            SMP19 { RwRwRegFieldBits }
        }
        PCSEL {
            0x20 RwReg;
            PCSEL0 { RwRwRegFieldBit }
            PCSEL1 { RwRwRegFieldBit }
            PCSEL2 { RwRwRegFieldBit }
            PCSEL3 { RwRwRegFieldBit }
            PCSEL4 { RwRwRegFieldBit }
            PCSEL5 { RwRwRegFieldBit }
            PCSEL6 { RwRwRegFieldBit }
            PCSEL7 { RwRwRegFieldBit }
            PCSEL8 { RwRwRegFieldBit }
            PCSEL9 { RwRwRegFieldBit }
            PCSEL10 { RwRwRegFieldBit }
            PCSEL11 { RwRwRegFieldBit }
            PCSEL12 { RwRwRegFieldBit }
            PCSEL13 { RwRwRegFieldBit }
            PCSEL14 { RwRwRegFieldBit }
            PCSEL15 { RwRwRegFieldBit }
            PCSEL16 { RwRwRegFieldBit }
            PCSEL17 { RwRwRegFieldBit }
            PCSEL18 { RwRwRegFieldBit }
            PCSEL19 { RwRwRegFieldBit }
        }
        LTR1 {
            0x20 RwReg;
            LTR1 { RwRwRegFieldBits }
        }
        HTR1 {
            0x20 RwReg;
            HTR1 { RwRwRegFieldBits }
        }
        SQR1 {
            0x20 RwReg;
            L { RwRwRegFieldBits }
            SQ1 { RwRwRegFieldBits }
            SQ2 { RwRwRegFieldBits }
            SQ3 { RwRwRegFieldBits }
            SQ4 { RwRwRegFieldBits }
        }
        SQR2 {
            0x20 RwReg;
            SQ5 { RwRwRegFieldBits }
            SQ6 { RwRwRegFieldBits }
            SQ7 { RwRwRegFieldBits }
            SQ8 { RwRwRegFieldBits }
            SQ9 { RwRwRegFieldBits }
        }
        SQR3 {
            0x20 RwReg;
            SQ10 { RwRwRegFieldBits }
            SQ11 { RwRwRegFieldBits }
            SQ12 { RwRwRegFieldBits }
            SQ13 { RwRwRegFieldBits }
            SQ14 { RwRwRegFieldBits }
        }
        SQR4 {
            0x20 RwReg;
            SQ15 { RwRwRegFieldBits }
            SQ16 { RwRwRegFieldBits }
        }
        DR {
            0x20 RoReg;
            RDATA { RoRoRegFieldBits }
        }
        JSQR {
            0x20 RwReg;
            JL { RwRwRegFieldBits }
            JEXTSEL { RwRwRegFieldBits }
            JEXTEN { RwRwRegFieldBits }
            JSQ1 { RwRwRegFieldBits }
            JSQ2 { RwRwRegFieldBits }
            JSQ3 { RwRwRegFieldBits }
            JSQ4 { RwRwRegFieldBits }
        }
        OFR1 {
            0x20 RwReg;
            OFFSET1 { RwRwRegFieldBits }
            OFFSET1_CH { RwRwRegFieldBits }
            SSATE { RwRwRegFieldBit }
        }
        OFR2 {
            0x20 RwReg;
            OFFSET2 { RwRwRegFieldBits }
            OFFSET2_CH { RwRwRegFieldBits }
            SSATE { RwRwRegFieldBit }
        }
        OFR3 {
            0x20 RwReg;
            OFFSET3 { RwRwRegFieldBits }
            OFFSET3_CH { RwRwRegFieldBits }
            SSATE { RwRwRegFieldBit }
        }
        OFR4 {
            0x20 RwReg;
            OFFSET4 { RwRwRegFieldBits }
            OFFSET4_CH { RwRwRegFieldBits }
            SSATE { RwRwRegFieldBit }
        }
        JDR1 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR2 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR3 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        JDR4 {
            0x20 RoReg;
            JDATA { RoRoRegFieldBits }
        }
        AWD2CR {
            0x20 RwReg;
            AWD2CH { RwRwRegFieldBits }
        }
        AWD3CR {
            0x20 RwReg;
            AWD3CH { RwRwRegFieldBits }
        }
        LTR2 {
            0x20 RwReg;
            LTR2 { RwRwRegFieldBits }
        }
        HTR2 {
            0x20 RwReg;
            HTR2 { RwRwRegFieldBits }
        }
        LTR3 {
            0x20 RwReg;
            LTR3 { RwRwRegFieldBits }
        }
        HTR3 {
            0x20 RwReg;
            HTR3 { RwRwRegFieldBits }
        }
        DIFSEL {
            0x20 RwReg;
            DIFSEL { RwRwRegFieldBits }
        }
        CALFACT {
            0x20 RwReg;
            CALFACT_D { RwRwRegFieldBits }
            CALFACT_S { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_adc {
    (
        $adc_macro_doc:expr,
        $adc_macro:ident,
        $adc_ty_doc:expr,
        $adc_ty:ident,
        $adc:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $adcen:ident,
        $adcrst:ident,
        $adcsmen:ident,
    ) => {
        periph::map! {
            #[doc = $adc_macro_doc]
            pub macro $adc_macro;

            #[doc = $adc_ty_doc]
            pub struct $adc_ty;

            impl AdcMap for $adc_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    ADCEN { $adcen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    ADCRST { $adcrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    ADCSMEN { $adcsmen }
                }
                DCCIPR {
                    D3CCIPR Shared;
                    ADCSEL { ADCSEL }
                }
            }

            ADC {
                $adc;
                ISR {
                    ISR;
                    ADRDY { ADRDY }
                    EOSMP { EOSMP }
                    EOC { EOC }
                    EOS { EOS }
                    OVR { OVR }
                    JEOC { JEOC }
                    JEOS { JEOS }
                    AWD1 { AWD1 }
                    AWD2 { AWD2 }
                    AWD3 { AWD3 }
                    JQOVF { JQOVF }
                    LDORDY { LDORDY }
                }
                IER {
                    IER;
                    ADRDYIE { ADRDYIE }
                    EOSMPIE { EOSMPIE }
                    EOCIE { EOCIE }
                    EOSIE { EOSIE }
                    OVRIE { OVRIE }
                    JEOCIE { JEOCIE }
                    JEOSIE { JEOSIE }
                    AWD1IE { AWD1IE }
                    AWD2IE { AWD2IE }
                    AWD3IE { AWD3IE }
                    JQOVFIE { JQOVFIE }
                }
                CR {
                    CR;
                    ADEN { ADEN }
                    ADDIS { ADDIS }
                    ADSTART { ADSTART }
                    JADSTART { JADSTART }
                    ADSTP { ADSTP }
                    JADSTP { JADSTP }
                    BOOST { BOOST }
                    ADCALLIN { ADCALLIN }
                    LINCALRDYW1 { LINCALRDYW1 }
                    LINCALRDYW2 { LINCALRDYW2 }
                    LINCALRDYW3 { LINCALRDYW3 }
                    LINCALRDYW4 { LINCALRDYW4 }
                    LINCALRDYW5 { LINCALRDYW5 }
                    LINCALRDYW6 { LINCALRDYW6 }
                    ADVREGEN { ADVREGEN }
                    DEEPPWD { DEEPPWD }
                    ADCALDIF { ADCALDIF }
                    ADCAL { ADCAL }
                }
                CFGR {
                    CFGR;
                    DMNGT { DMNGT }
                    RES { RES }
                    EXTSEL { EXTSEL }
                    EXTEN { EXTEN }
                    OVRMOD { OVRMOD }
                    CONT { CONT }
                    AUTDLY { AUTDLY }
                    DISCEN { DISCEN }
                    DISCNUM { DISCNUM }
                    JDISCEN { JDISCEN }
                    JQM { JQM }
                    AWD1SGL { AWD1SGL }
                    AWD1EN { AWD1EN }
                    JAWD1EN { JAWD1EN }
                    JAUTO { JAUTO }
                    AWDCH1CH { AWDCH1CH }
                    JQDIS { JQDIS }
                }
                CFGR2 {
                    CFGR2;
                    JOVSE { JOVSE }
                    ROVSE { ROVSE }
                    OVSS { OVSS }
                    TROVS { TROVS }
                    ROVSM { ROVSM }
                    RSHIFT1 { RSHIFT1 }
                    RSHIFT2 { RSHIFT2 }
                    RSHIFT3 { RSHIFT3 }
                    RSHIFT4 { RSHIFT4 }
                    OSVR { OSVR }
                    LSHIFT { LSHIFT }
                }
                SMPR1 {
                    SMPR1;
                    SMP0 { SMP0 }
                    SMP1 { SMP1 }
                    SMP2 { SMP2 }
                    SMP3 { SMP3 }
                    SMP4 { SMP4 }
                    SMP5 { SMP5 }
                    SMP6 { SMP6 }
                    SMP7 { SMP7 }
                    SMP8 { SMP8 }
                    SMP9 { SMP9 }
                }
                SMPR2 {
                    SMPR2;
                    SMP10 { SMP10 }
                    SMP11 { SMP11 }
                    SMP12 { SMP12 }
                    SMP13 { SMP13 }
                    SMP14 { SMP14 }
                    SMP15 { SMP15 }
                    SMP16 { SMP16 }
                    SMP17 { SMP17 }
                    SMP18 { SMP18 }
                    SMP19 { SMP19 }
                }
                PCSEL {
                    PCSEL;
                    PCSEL0 { PCSEL0 }
                    PCSEL1 { PCSEL1 }
                    PCSEL2 { PCSEL2 }
                    PCSEL3 { PCSEL3 }
                    PCSEL4 { PCSEL4 }
                    PCSEL5 { PCSEL5 }
                    PCSEL6 { PCSEL6 }
                    PCSEL7 { PCSEL7 }
                    PCSEL8 { PCSEL8 }
                    PCSEL9 { PCSEL9 }
                    PCSEL10 { PCSEL10 }
                    PCSEL11 { PCSEL11 }
                    PCSEL12 { PCSEL12 }
                    PCSEL13 { PCSEL13 }
                    PCSEL14 { PCSEL14 }
                    PCSEL15 { PCSEL15 }
                    PCSEL16 { PCSEL16 }
                    PCSEL17 { PCSEL17 }
                    PCSEL18 { PCSEL18 }
                    PCSEL19 { PCSEL19 }
                }
                LTR1 {
                    LTR1;
                    LTR1 { LTR1 }
                }
                HTR1 {
                    HTR1;
                    HTR1 { HTR1 }
                }
                SQR1 {
                    SQR1;
                    L { L }
                    SQ1 { SQ1 }
                    SQ2 { SQ2 }
                    SQ3 { SQ3 }
                    SQ4 { SQ4 }
                }
                SQR2 {
                    SQR2;
                    SQ5 { SQ5 }
                    SQ6 { SQ6 }
                    SQ7 { SQ7 }
                    SQ8 { SQ8 }
                    SQ9 { SQ9 }
                }
                SQR3 {
                    SQR3;
                    SQ10 { SQ10 }
                    SQ11 { SQ11 }
                    SQ12 { SQ12 }
                    SQ13 { SQ13 }
                    SQ14 { SQ14 }
                }
                SQR4 {
                    SQR4;
                    SQ15 { SQ15 }
                    SQ16 { SQ16 }
                }
                DR {
                    DR;
                    RDATA { RDATA }
                }
                JSQR {
                    JSQR;
                    JL { JL }
                    JEXTSEL { JEXTSEL }
                    JEXTEN { JEXTEN }
                    JSQ1 { JSQ1 }
                    JSQ2 { JSQ2 }
                    JSQ3 { JSQ3 }
                    JSQ4 { JSQ4 }
                }
                OFR1 {
                    OFR1;
                    OFFSET1 { OFFSET1 }
                    OFFSET1_CH { OFFSET1_CH }
                    SSATE { SSATE }
                }
                OFR2 {
                    OFR2;
                    OFFSET2 { OFFSET2 }
                    OFFSET2_CH { OFFSET2_CH }
                    SSATE { SSATE }
                }
                OFR3 {
                    OFR3;
                    OFFSET3 { OFFSET3 }
                    OFFSET3_CH { OFFSET3_CH }
                    SSATE { SSATE }
                }
                OFR4 {
                    OFR4;
                    OFFSET4 { OFFSET4 }
                    OFFSET4_CH { OFFSET4_CH }
                    SSATE { SSATE }
                }
                JDR1 {
                    JDR1;
                    JDATA { JDATA }
                }
                JDR2 {
                    JDR2;
                    JDATA { JDATA }
                }
                JDR3 {
                    JDR3;
                    JDATA { JDATA }
                }
                JDR4 {
                    JDR4;
                    JDATA { JDATA }
                }
                AWD2CR {
                    AWD2CR;
                    AWD2CH { AWD2CH }
                }
                AWD3CR {
                    AWD3CR;
                    AWD3CH { AWD3CH }
                }
                LTR2 {
                    LTR2;
                    LTR2 { LTR2 }
                }
                HTR2 {
                    HTR2;
                    HTR2 { HTR2 }
                }
                LTR3 {
                    LTR3;
                    LTR3 { LTR3 }
                }
                HTR3 {
                    HTR3;
                    HTR3 { HTR3 }
                }
                DIFSEL {
                    DIFSEL;
                    DIFSEL { DIFSEL }
                }
                CALFACT {
                    CALFACT;
                    CALFACT_D { CALFACT_D }
                    CALFACT_S { CALFACT_S }
                }
            }
        }
    };
}

map_adc! {
    "Extracts ADC1 register tokens.",
    periph_adc1,
    "ADC1 peripheral variant.",
    Adc1,
    ADC1,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    ADC12EN,
    ADC12RST,
    ADC12LPEN,
}

map_adc! {
    "Extracts ADC2 register tokens.",
    periph_adc2,
    "ADC2 peripheral variant.",
    Adc2,
    ADC2,
    AHB1ENR,
    AHB1RSTR,
    AHB1LPENR,
    ADC12EN,
    ADC12RST,
    ADC12LPEN,
}

map_adc! {
    "Extracts ADC3 register tokens.",
    periph_adc3,
    "ADC3 peripheral variant.",
    Adc3,
    ADC3,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    ADC3EN,
    ADC3RST,
    ADC3LPEN,
}
