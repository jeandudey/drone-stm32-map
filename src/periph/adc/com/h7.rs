//! Analog-to-digital converters common registers.
//!
//! For STM32L4+ series of ultra-low-power MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic ADC Common peripheral variant.
    pub trait AdcComMap {}

    /// Generic ADC Common peripheral.
    pub struct AdcComPeriph;

    RCC {
        BUSRSTR {
            0x20 RwReg Shared;
            ADCRST { RwRwRegFieldBit }
        }
    }

    ADC_Common {
        CSR {
            0x20 RoReg;
            ADRDY_MST { RoRoRegFieldBit }
            EOSMP_MST { RoRoRegFieldBit }
            EOC_MST { RoRoRegFieldBit }
            EOS_MST { RoRoRegFieldBit }
            OVR_MST { RoRoRegFieldBit }
            JEOC_MST { RoRoRegFieldBit }
            JEOS_MST { RoRoRegFieldBit }
            AWD1_MST { RoRoRegFieldBit }
            AWD2_MST { RoRoRegFieldBit }
            AWD3_MST { RoRoRegFieldBit }
            JQOVF_MST { RoRoRegFieldBit }
            ADRDY_SLV { RoRoRegFieldBit }
            EOSMP_SLV { RoRoRegFieldBit }
            EOC_SLV { RoRoRegFieldBit }
            EOS_SLV { RoRoRegFieldBit }
            OVR_SLV { RoRoRegFieldBit }
            JEOC_SLV { RoRoRegFieldBit }
            JEOS_SLV { RoRoRegFieldBit }
            AWD1_SLV { RoRoRegFieldBit }
            AWD2_SLV { RoRoRegFieldBit }
            AWD3_SLV { RoRoRegFieldBit }
            JQOVF_SLV { RoRoRegFieldBit }
        }
        CCR {
            0x20 RwReg;
            DUAL { RwRwRegFieldBits }
            DELAY { RwRwRegFieldBits }
            DAMDF { RwRwRegFieldBits }
            CKMODE { RwRwRegFieldBits }
            PRESC { RwRwRegFieldBits }
            VREFEN { RwRwRegFieldBit }
            TSEN { RwRwRegFieldBit }
            VBATEN { RwRwRegFieldBit }
        }
        CDR {
            0x20 RoReg;
            RDATA_MST { RoRoRegFieldBits }
            RDATA_SLV { RoRoRegFieldBits }
        }
        CDR2 {
            0x20 RoReg;
            RDATA_ALT { RoRoRegFieldBits }
        }
    }
}

macro_rules! map_adc_common {
    (
        $adc_com_macro_doc:expr,
        $adc_com_macro:ident,
        $adc_com_ty_doc:expr,
        $adc_com_ty:ident,
        $adc_com:ident,
        $busrstr:ident,
        $adcrst:ident,
    ) => {
        periph::map! {
            #[doc = $adc_com_macro_doc]
            pub macro $adc_com_macro;

            #[doc = $adc_com_ty_doc]
            pub struct $adc_com_ty;

            impl AdcComMap for $adc_com_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSRSTR {
                    $busrstr Shared;
                    ADCRST { $adcrst }
                }
            }

            ADC_Common {
                $adc_com;

                CSR {
                    CSR;
                    ADRDY_MST { ADRDY_MST }
                    EOSMP_MST { EOSMP_MST }
                    EOC_MST { EOC_MST }
                    EOS_MST { EOS_MST }
                    OVR_MST { OVR_MST }
                    JEOC_MST { JEOC_MST }
                    JEOS_MST { JEOS_MST }
                    AWD1_MST { AWD1_MST }
                    AWD2_MST { AWD2_MST }
                    AWD3_MST { AWD3_MST }
                    JQOVF_MST { JQOVF_MST }
                    ADRDY_SLV { ADRDY_SLV }
                    EOSMP_SLV { EOSMP_SLV }
                    EOC_SLV { EOC_SLV }
                    EOS_SLV { EOS_SLV }
                    OVR_SLV { OVR_SLV }
                    JEOC_SLV { JEOC_SLV }
                    JEOS_SLV { JEOS_SLV }
                    AWD1_SLV { AWD1_SLV }
                    AWD2_SLV { AWD2_SLV }
                    AWD3_SLV { AWD3_SLV }
                    JQOVF_SLV { JQOVF_SLV }
                }
                CCR {
                    CCR;
                    DUAL { DUAL }
                    DELAY { DELAY }
                    DAMDF { DAMDF }
                    CKMODE { CKMODE }
                    PRESC { PRESC }
                    VREFEN { VREFEN }
                    TSEN { TSEN }
                    VBATEN { VBATEN }
                }
                CDR {
                    CDR;
                    RDATA_MST { RDATA_MST }
                    RDATA_SLV { RDATA_SLV }
                }
                CDR2 {
                    CDR2;
                    RDATA_ALT { RDATA_ALT }
                }
            }
        }
    };
}

map_adc_common! {
    "Extracts ADC1 and ADC2 common register tokens",
    periph_adc12_common,
    "ADC1 and ADC2 common peripheral variant.",
    Adc12ComPeriph,
    ADC12_Common,
    AHB1RSTR,
    ADC12RST,
}

map_adc_common! {
    "Extracts ADC3 ommon register tokens",
    periph_adc3_common,
    "ADC3 common peripheral variant.",
    Adc3ComPeriph,
    ADC3_Common,
    AHB4RSTR,
    ADC3RST,
}
