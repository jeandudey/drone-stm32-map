//! Inter-Integrated Circuit.
//!
//! For STM32H7 Series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic I2C peripheral variant.
    pub trait I2CMap {}

    /// Generic I2C peripheral.
    pub struct I2CPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            I2CEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            I2CRST { RwRwRegFieldBit }
        }
    }

    I2C {
        CR1 {
            0x20 RwReg;
            PE { RwRwRegFieldBit }
            TXIE { RwRwRegFieldBit }
            RXIE { RwRwRegFieldBit }
            ADDRIE { RwRwRegFieldBit }
            NACKIE { RwRwRegFieldBit }
            STOPIE { RwRwRegFieldBit }
            TCIE { RwRwRegFieldBit }
            ERRIE { RwRwRegFieldBit }
            DNF { RwRwRegFieldBits }
            ANFOFF { RwRwRegFieldBit }
            TXDMAEN { RwRwRegFieldBit }
            RXDMAEN { RwRwRegFieldBit }
            SBC { RwRwRegFieldBit }
            NOSTRETCH { RwRwRegFieldBit }
            WUPEN { RwRwRegFieldBit }
            GCEN { RwRwRegFieldBit }
            SMBHEN { RwRwRegFieldBit }
            SMBDEN { RwRwRegFieldBit }
            ALERTEN { RwRwRegFieldBit }
            PECEN { RwRwRegFieldBit }
        }
        CR2 {
            0x20 RwReg;
            PECBYTE { RwRwRegFieldBit }
            AUTOEND { RwRwRegFieldBit }
            RELOAD { RwRwRegFieldBit }
            NBYTES { RwRwRegFieldBits }
            NACK { RwRwRegFieldBit }
            STOP { RwRwRegFieldBit }
            START { RwRwRegFieldBit }
            HEAD10R { RwRwRegFieldBit }
            ADD10 { RwRwRegFieldBit }
            RD_WRN { RwRwRegFieldBit }
            SADD9 { RwRwRegFieldBit }
            SADD8 { RwRwRegFieldBit }
            SADD7 { RwRwRegFieldBit }
            SADD6 { RwRwRegFieldBit }
            SADD5 { RwRwRegFieldBit }
            SADD4 { RwRwRegFieldBit }
            SADD3 { RwRwRegFieldBit }
            SADD2 { RwRwRegFieldBit }
            SADD1 { RwRwRegFieldBit }
            SADD0 { RwRwRegFieldBit }
        }
        OAR1 {
            0x20 RwReg;
            OA1 { RwRwRegFieldBits }
            OA1MODE { RwRwRegFieldBit }
            OA1EN { RwRwRegFieldBit }
        }
        OAR2 {
            0x20 RwReg;
            OA2 { RwRwRegFieldBits }
            OA2MSK { RwRwRegFieldBits }
            OA2EN { RwRwRegFieldBit }
        }
        TIMINGR {
            0x20 RwReg;
            SCLL { RwRwRegFieldBits }
            SCLH { RwRwRegFieldBits }
            SDADEL { RwRwRegFieldBits }
            SCLDEL { RwRwRegFieldBits }
            PRESC { RwRwRegFieldBits }
        }
        TIMEOUTR {
            0x20 RwReg;
            TIMEOUTA { RwRwRegFieldBits }
            TIDLE { RwRwRegFieldBit }
            TIMOUTEN { RwRwRegFieldBit }
            TIMEOUTB { RwRwRegFieldBits }
            TEXTEN { RwRwRegFieldBit }
        }
        ISR {
            0x20 RwReg;
            ADDCODE { RoRwRegFieldBits }
            DIR { RoRwRegFieldBit }
            BUSY { RoRwRegFieldBit }
            ALERT { RoRwRegFieldBit }
            TIMEOUT { RoRwRegFieldBit }
            PECERR { RoRwRegFieldBit }
            OVR { RoRwRegFieldBit }
            ARLO { RoRwRegFieldBit }
            BERR { RoRwRegFieldBit }
            TCR { RoRwRegFieldBit }
            TC { RoRwRegFieldBit }
            STOPF { RoRwRegFieldBit }
            NACKF { RoRwRegFieldBit }
            ADDR { RoRwRegFieldBit }
            RXNE { RoRwRegFieldBit }
            TXIS { RwRwRegFieldBit }
            TXE { RwRwRegFieldBit }
        }
        ICR {
            0x20 WoReg;
            ALERTCF { WoWoRegFieldBit }
            TIMOUTCF { WoWoRegFieldBit }
            PECCF { WoWoRegFieldBit }
            OVRCF { WoWoRegFieldBit }
            ARLOCF { WoWoRegFieldBit }
            BERRCF { WoWoRegFieldBit }
            STOPCF { WoWoRegFieldBit }
            NACKCF { WoWoRegFieldBit }
            ADDRCF { WoWoRegFieldBit }
        }
        PECR {
            0x20 RoReg;
            PEC { RoRoRegFieldBits }
        }
        RXDR {
            0x20 RoReg;
            RXDATA { RoRoRegFieldBits }
        }
        TXDR {
            0x20 RwReg;
            TXDATA { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_i2c {
    (
        $i2c_macro_doc:expr,
        $i2c_macro:ident,
        $i2c_ty_doc:expr,
        $i2c_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $i2c:ident,
        $i2cen:ident,
        $i2crst:ident,
    ) => {
        periph::map! {
            #[doc = $i2c_macro_doc]
            pub macro $i2c_macro;

            #[doc = $i2c_ty_doc]
            pub struct $i2c_ty;

            impl I2CMap for $i2c_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    I2CEN { $i2cen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    I2CRST { $i2crst }
                }
            }

            I2C {
                $i2c;
                CR1 {
                    CR1;
                    PE { PE }
                    TXIE { TXIE }
                    RXIE { RXIE }
                    ADDRIE { ADDRIE }
                    NACKIE { NACKIE }
                    STOPIE { STOPIE }
                    TCIE { TCIE }
                    ERRIE { ERRIE }
                    DNF { DNF }
                    ANFOFF { ANFOFF }
                    TXDMAEN { TXDMAEN }
                    RXDMAEN { RXDMAEN }
                    SBC { SBC }
                    NOSTRETCH { NOSTRETCH }
                    WUPEN { WUPEN }
                    GCEN { GCEN }
                    SMBHEN { SMBHEN }
                    SMBDEN { SMBDEN }
                    ALERTEN { ALERTEN }
                    PECEN { PECEN }
                }
                CR2 {
                    CR2;
                    PECBYTE { PECBYTE }
                    AUTOEND { AUTOEND }
                    RELOAD { RELOAD }
                    NBYTES { NBYTES }
                    NACK { NACK }
                    STOP { STOP }
                    START { START }
                    HEAD10R { HEAD10R }
                    ADD10 { ADD10 }
                    RD_WRN { RD_WRN }
                    SADD9 { SADD9 }
                    SADD8 { SADD8 }
                    SADD7 { SADD7 }
                    SADD6 { SADD6 }
                    SADD5 { SADD5 }
                    SADD4 { SADD4 }
                    SADD3 { SADD3 }
                    SADD2 { SADD2 }
                    SADD1 { SADD1 }
                    SADD0 { SADD0 }
                }
                OAR1 {
                    OAR1;
                    OA1 { OA1 }
                    OA1MODE { OA1MODE }
                    OA1EN { OA1EN }
                }
                OAR2 {
                    OAR2;
                    OA2 { OA2 }
                    OA2MSK { OA2MSK }
                    OA2EN { OA2EN }
                }
                TIMINGR {
                    TIMINGR;
                    SCLL { SCLL }
                    SCLH { SCLH }
                    SDADEL { SDADEL }
                    SCLDEL { SCLDEL }
                    PRESC { PRESC }
                }
                TIMEOUTR {
                    TIMEOUTR;
                    TIMEOUTA { TIMEOUTA }
                    TIDLE { TIDLE }
                    TIMOUTEN { TIMOUTEN }
                    TIMEOUTB { TIMEOUTB }
                    TEXTEN { TEXTEN }
                }
                ISR {
                    ISR;
                    ADDCODE { ADDCODE }
                    DIR { DIR }
                    BUSY { BUSY }
                    ALERT { ALERT }
                    TIMEOUT { TIMEOUT }
                    PECERR { PECERR }
                    OVR { OVR }
                    ARLO { ARLO }
                    BERR { BERR }
                    TCR { TCR }
                    TC { TC }
                    STOPF { STOPF }
                    NACKF { NACKF }
                    ADDR { ADDR }
                    RXNE { RXNE }
                    TXIS { TXIS }
                    TXE { TXE }
                }
                ICR {
                    ICR;
                    ALERTCF { ALERTCF }
                    TIMOUTCF { TIMOUTCF }
                    PECCF { PECCF }
                    OVRCF { OVRCF }
                    ARLOCF { ARLOCF }
                    BERRCF { BERRCF }
                    STOPCF { STOPCF }
                    NACKCF { NACKCF }
                    ADDRCF { ADDRCF }
                }
                PECR {
                    PECR;
                    PEC { PEC }
                }
                RXDR {
                    RXDR;
                    RXDATA { RXDATA }
                }
                TXDR {
                    TXDR;
                    TXDATA { TXDATA }
                }
            }
        }
    };
}

map_i2c! {
    "Extracts I2C1 register tokens.",
    periph_i2c1,
    "I2C1 peripheral variant.",
    I2C1,
    APB1LENR,
    APB1LRSTR,
    I2C1,
    I2C1EN,
    I2C1RST,
}

map_i2c! {
    "Extracts I2C2 register tokens.",
    periph_i2c2,
    "I2C2 peripheral variant.",
    I2C2,
    APB1LENR,
    APB1LRSTR,
    I2C2,
    I2C2EN,
    I2C2RST,
}

map_i2c! {
    "Extracts I2C3 register tokens.",
    periph_i2c3,
    "I2C3 peripheral variant.",
    I2C3,
    APB1LENR,
    APB1LRSTR,
    I2C3,
    I2C3EN,
    I2C3RST,
}
