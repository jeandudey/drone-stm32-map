//! Serial Peripheral Interface.
//!
//! For STM32H7 series MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic SPI peripheral variant.
    pub trait SpiMap {}

    /// Generic SPI peripheral.
    pub struct SpiPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            SPIEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            SPIRST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            SPISMEN { RwRwRegFieldBit }
        }
    }
    SPI {
        CR1 {
            0x20 RwReg;
            SPE { RwRwRegFieldBit }
            MASRX { RwRwRegFieldBit }
            CSTART { RoRwRegFieldBit }
            CSUSP { WoRwRegFieldBit }
            HDDIR { RwRwRegFieldBit }
            SSI { RwRwRegFieldBit }
            CRC33_17 { RwRwRegFieldBit }
            RCRCINI { RwRwRegFieldBit }
            TCRCINI { RwRwRegFieldBit }
            IOLOCK { RoRwRegFieldBit }
        }
        CR2 {
            0x20 RwReg;
            TSER { RoRwRegFieldBits }
            TSIZE { RwRwRegFieldBits }
        }
        CFG1 {
            0x20 RwReg;
            DSIZE { RwRwRegFieldBits }
            FTHLV { RwRwRegFieldBits }
            UDRCFG { RwRwRegFieldBits }
            UDRDET { RwRwRegFieldBits }
            RXDMAEN { RwRwRegFieldBit }
            TXDMAEN { RwRwRegFieldBit }
            CRCSIZE { RwRwRegFieldBits }
            CRCEN { RwRwRegFieldBit }
            MBR { RwRwRegFieldBits }
        }
        CFG2 {
            0x20 RwReg;
            MSSI { RwRwRegFieldBits }
            MIDI { RwRwRegFieldBits }
            IOSWP { RwRwRegFieldBit }
            COMM { RwRwRegFieldBits }
            SP { RwRwRegFieldBits }
            MASTER { RwRwRegFieldBit }
            LSBFRST { RwRwRegFieldBit }
            CPHA { RwRwRegFieldBit }
            CPOL { RwRwRegFieldBit }
            SSM { RwRwRegFieldBit }
            SSIOP { RwRwRegFieldBit }
            SSOE { RwRwRegFieldBit }
            SSOM { RwRwRegFieldBit }
            AFCNTR { RwRwRegFieldBit }
        }
        IER {
            0x20 RwReg;
            RXPIE { RwRwRegFieldBit }
            TXPIE { RoRwRegFieldBit }
            DXPIE { RoRwRegFieldBit }
            EOTIE { RwRwRegFieldBit }
            TXTFIE { RwRwRegFieldBit }
            UDRIE { RwRwRegFieldBit }
            OVRIE { RwRwRegFieldBit }
            CRCEIE { RwRwRegFieldBit }
            TIFREIE  { RwRwRegFieldBit }
            MODFIE { RwRwRegFieldBit }
            TSERFIE { RwRwRegFieldBit }
        }
        SR {
            0x20 RoReg;
            RXP { RoRoRegFieldBit }
            TXP { RoRoRegFieldBit }
            DXP { RoRoRegFieldBit }
            EOT { RoRoRegFieldBit }
            TXTF { RoRoRegFieldBit }
            UDR { RoRoRegFieldBit }
            OVR { RoRoRegFieldBit }
            CRCE { RoRoRegFieldBit }
            TIFRE { RoRoRegFieldBit }
            MODF { RoRoRegFieldBit }
            TSERF { RoRoRegFieldBit }
            SUSP { RoRoRegFieldBit }
            TXC { RoRoRegFieldBit }
            RXPLVL { RoRoRegFieldBits }
            RXWNE { RoRoRegFieldBit }
            CTSIZE { RoRoRegFieldBits }
        }
        IFCR {
            0x20 WoReg;
            EOTC { WoWoRegFieldBit }
            TXTFC { WoWoRegFieldBit }
            UDRC { WoWoRegFieldBit }
            OVRC { WoWoRegFieldBit }
            CRCEC { WoWoRegFieldBit }
            TIFREC { WoWoRegFieldBit }
            MODFC { WoWoRegFieldBit }
            TSERFC { WoWoRegFieldBit }
            SUSPC { WoWoRegFieldBit }
        }
        TXDR {
            0x20 WoReg;
            TXDR { WoWoRegFieldBits }
        }
        RXDR {
            0x20 RoReg;
            RXDR { RoRoRegFieldBits }
        }
        CRCPOLY {
            0x20 RwReg;
            CRCPOLY { RwRwRegFieldBits }
        }
        RXCRC {
            0x20 RoReg;
            RXCRC { RoRoRegFieldBits }
        }
        TXCRC {
            0x20 RoReg;
            TXCRC { RoRoRegFieldBits }
        }
        UDRDR {
            0x20 RwReg;
            UDRDR { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_spi {
    (
        $spi_macro_doc:expr,
        $spi_macro:ident,
        $spi_ty_doc:expr,
        $spi_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $spien:ident,
        $spirst:ident,
        $spismen:ident,
        $spi:ident,
    ) => {
        periph::map! {
            #[doc = $spi_macro_doc]
            pub macro $spi_macro;

            #[doc = $spi_ty_doc]
            pub struct $spi_ty;

            impl SpiMap for $spi_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    SPIEN { $spien }
                }
                BUSRSTR {
                    $busrstr Shared;
                    SPIRST { $spirst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    SPISMEN { $spismen }
                }
            }
            SPI {
                $spi;
                CR1 {
                    CR1;
                    SPE { SPE }
                    MASRX { MASRX }
                    CSTART { CSTART }
                    CSUSP { CSUSP }
                    HDDIR { HDDIR }
                    SSI { SSI }
                    CRC33_17 { CRC33_17 }
                    RCRCINI { RCRCINI }
                    TCRCINI { TCRCINI }
                    IOLOCK { IOLOCK }
                }
                CR2 {
                    CR2;
                    TSER { TSER }
                    TSIZE { TSIZE }
                }
                CFG1 {
                    CFG1;
                    DSIZE { DSIZE }
                    FTHLV { FTHLV }
                    UDRCFG { UDRCFG }
                    UDRDET { UDRDET }
                    RXDMAEN { RXDMAEN }
                    TXDMAEN { TXDMAEN }
                    CRCSIZE { CRCSIZE }
                    CRCEN { CRCEN }
                    MBR { MBR }
                }
                CFG2 {
                    CFG2;
                    MSSI { MSSI }
                    MIDI { MIDI }
                    IOSWP { IOSWP }
                    COMM { COMM }
                    SP { SP }
                    MASTER { MASTER }
                    LSBFRST { LSBFRST }
                    CPHA { CPHA }
                    CPOL { CPOL }
                    SSM { SSM }
                    SSIOP { SSIOP }
                    SSOE { SSOE }
                    SSOM { SSOM }
                    AFCNTR { AFCNTR }
                }
                IER {
                    IER;
                    RXPIE { RXPIE }
                    TXPIE { TXPIE }
                    DXPIE { DXPIE }
                    EOTIE { EOTIE }
                    TXTFIE { TXTFIE }
                    UDRIE { UDRIE }
                    OVRIE { OVRIE }
                    CRCEIE { CRCEIE }
                    TIFREIE  { TIFREIE }
                    MODFIE { MODFIE }
                    TSERFIE { TSERFIE }
                }
                SR {
                    SR;
                    RXP { RXP }
                    TXP { TXP }
                    DXP { DXP }
                    EOT { EOT }
                    TXTF { TXTF }
                    UDR { UDR }
                    OVR { OVR }
                    CRCE { CRCE }
                    TIFRE { TIFRE }
                    MODF { MODF }
                    TSERF { TSERF }
                    SUSP { SUSP }
                    TXC { TXC }
                    RXPLVL { RXPLVL }
                    RXWNE { RXWNE }
                    CTSIZE { CTSIZE }
                }
                IFCR {
                    IFCR;
                    EOTC { EOTC }
                    TXTFC { TXTFC }
                    UDRC { UDRC }
                    OVRC { OVRC }
                    CRCEC { CRCEC }
                    TIFREC { TIFREC }
                    MODFC { MODFC }
                    TSERFC { TSERFC }
                    SUSPC { SUSPC }
                }
                TXDR {
                    TXDR;
                    TXDR { TXDR }
                }
                RXDR {
                    RXDR;
                    RXDR { RXDR }
                }
                CRCPOLY {
                    CRCPOLY;
                    CRCPOLY { CRCPOLY }
                }
                RXCRC {
                    RXCRC;
                    RXCRC { RXCRC }
                }
                TXCRC {
                    TXCRC;
                    TXCRC { TXCRC }
                }
                UDRDR {
                    UDRDR;
                    UDRDR { UDRDR }
                }
            }
        }
    };
}

map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "SPI1 peripheral variant.",
    Spi1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI1EN,
    SPI1RST,
    SPI1LPEN,
    SPI1,
}

map_spi! {
    "Extracts SPI2 register tokens.",
    periph_spi2,
    "SPI2 peripheral variant.",
    Spi2,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    SPI2EN,
    SPI2RST,
    SPI2LPEN,
    SPI2,
}

map_spi! {
    "Extracts SPI3 register tokens.",
    periph_spi3,
    "SPI3 peripheral variant.",
    Spi3,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    SPI3EN,
    SPI3RST,
    SPI3LPEN,
    SPI3,
}

map_spi! {
    "Extracts SPI4 register tokens.",
    periph_spi4,
    "SPI4 peripheral variant.",
    Spi4,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI4EN,
    SPI4RST,
    SPI4LPEN,
    SPI4,
}

map_spi! {
    "Extracts SPI5 register tokens.",
    periph_spi5,
    "SPI5 peripheral variant.",
    Spi5,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    SPI5EN,
    SPI5RST,
    SPI5LPEN,
    SPI5,
}

map_spi! {
    "Extracts SPI6 register tokens.",
    periph_spi6,
    "SPI6 peripheral variant.",
    Spi6,
    APB4ENR,
    APB4RSTR,
    APB4LPENR,
    SPI6EN,
    SPI6RST,
    SPI6LPEN,
    SPI6,
}
