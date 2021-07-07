//! Universal Asynchronous Receiver/Transmitter.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic UART peripheral variant.
    pub trait UartMap {}

    /// Generic UART peripheral.
    pub struct UartPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            UARTEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            UARTRST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            UARTSMEN { RwRwRegFieldBit }
        }
        DCCIPR {
            0x20 RwReg Shared;
            UARTSEL { RwRwRegFieldBits }
        }
    }

    UART {
        CR1 {
            0x20 RwReg;
            RXFFIE { RwRwRegFieldBit }
            TXFEIE { RwRwRegFieldBit }
            FIFOEN { RwRwRegFieldBit }
            M1 { RwRwRegFieldBit }
            EOBIE { RwRwRegFieldBit Option }
            RTOIE { RwRwRegFieldBit Option }
            DEAT { RwRwRegFieldBits }
            DEDT { RwRwRegFieldBits }
            OVER8 { RwRwRegFieldBit Option }
            CMIE { RwRwRegFieldBit }
            MME { RwRwRegFieldBit }
            M0 { RwRwRegFieldBit }
            WAKE { RwRwRegFieldBit }
            PCE { RwRwRegFieldBit }
            PS { RwRwRegFieldBit }
            PEIE { RwRwRegFieldBit }
            TXFNFIE { RwRwRegFieldBit }
            TCIE { RwRwRegFieldBit }
            RXFNEIE { RwRwRegFieldBit }
            IDLEIE { RwRwRegFieldBit }
            TE { RwRwRegFieldBit }
            RE { RwRwRegFieldBit }
            UESM { RwRwRegFieldBit }
            UE { RwRwRegFieldBit }
        }
        CR2 {
            0x20 RwReg;
            ADD { RwRwRegFieldBits }
            RTOEN { RwRwRegFieldBit Option }
            ABRMOD0 { RwRwRegFieldBit Option }
            ABRMOD1 { RwRwRegFieldBit Option }
            ABREN { RwRwRegFieldBit Option }
            MSBFIRST { RwRwRegFieldBit }
            DATAINV { RwRwRegFieldBit }
            TXINV { RwRwRegFieldBit }
            RXINV { RwRwRegFieldBit }
            SWAP { RwRwRegFieldBit }
            LINEN { RwRwRegFieldBit Option }
            STOP { RwRwRegFieldBits }
            CLKEN { RwRwRegFieldBit Option }
            CPHA { RwRwRegFieldBit Option }
            CPOL { RwRwRegFieldBit Option }
            LBCL { RwRwRegFieldBit Option }
            LBDIE { RwRwRegFieldBit Option }
            LBDL { RwRwRegFieldBit Option }
            ADDM7 { RwRwRegFieldBit }
            DIS_NSS { RwRwRegFieldBit Option }
            SLVEN { RwRwRegFieldBit Option }
        }
        CR3 {
            0x20 RwReg;
            TXFTCFG { RwRwRegFieldBits }
            RXFTIE { RwRwRegFieldBit }
            RXFTCFG { RwRwRegFieldBits }
            TCBGTIE { RwRwRegFieldBit Option }
            TXFTIE { RwRwRegFieldBit }
            WUFIE { RwRwRegFieldBit }
            WUS { RwRwRegFieldBits }
            SCARCNT { RwRwRegFieldBits Option }
            DEP { RwRwRegFieldBit }
            DEM { RwRwRegFieldBit }
            DDRE { RwRwRegFieldBit }
            OVRDIS { RwRwRegFieldBit }
            ONEBIT { RwRwRegFieldBit Option }
            CTSIE { RwRwRegFieldBit }
            CTSE { RwRwRegFieldBit }
            RTSE { RwRwRegFieldBit }
            DMAT { RwRwRegFieldBit }
            DMAR { RwRwRegFieldBit }
            SCEN { RwRwRegFieldBit Option }
            NACK { RwRwRegFieldBit Option }
            HDSEL { RwRwRegFieldBit }
            IRLP { RwRwRegFieldBit Option }
            IREN { RwRwRegFieldBit Option }
            EIE { RwRwRegFieldBit }
        }
        BRR {
            0x20 RwReg;
            BRR { RwRwRegFieldBits }
        }
        GTPR {
            0x20 RwReg;
            GT { RwRwRegFieldBits }
            PSC { RwRwRegFieldBits }
        }
        RTOR {
            0x20 RwReg Option;
            BLEN { RwRwRegFieldBits }
            RTO { RwRwRegFieldBits }
        }
        RQR {
            0x20 WoReg;
            TXFRQ { WoWoRegFieldBit }
            RXFRQ { WoWoRegFieldBit }
            MMRQ { WoWoRegFieldBit }
            SBKRQ { WoWoRegFieldBit }
            ABRRQ { WoWoRegFieldBit Option }
        }
        ISR {
            0x20 RoReg;
            TXFT { RoRoRegFieldBit }
            RXFT { RoRoRegFieldBit }
            TCBGT { RoRoRegFieldBit Option }
            RXFF { RoRoRegFieldBit }
            TXFE { RoRoRegFieldBit }
            REACK { RoRoRegFieldBit }
            TEACK { RoRoRegFieldBit }
            WUF { RoRoRegFieldBit }
            RWU { RoRoRegFieldBit }
            SBKF { RoRoRegFieldBit }
            CMF { RoRoRegFieldBit }
            BUSY { RoRoRegFieldBit }
            ABRF { RoRoRegFieldBit Option }
            ABRE { RoRoRegFieldBit Option }
            UDR { RoRoRegFieldBit Option }
            EOBF { RoRoRegFieldBit Option }
            RTOF { RoRoRegFieldBit Option }
            CTS { RoRoRegFieldBit }
            CTSIF { RoRoRegFieldBit }
            LBDF { RoRoRegFieldBit Option }
            TXE { RoRoRegFieldBit }
            TC { RoRoRegFieldBit }
            RXNE { RoRoRegFieldBit }
            IDLE { RoRoRegFieldBit }
            ORE { RoRoRegFieldBit }
            NE { RoRoRegFieldBit }
            FE { RoRoRegFieldBit }
            PE { RoRoRegFieldBit }
        }
        ICR {
            0x20 WoReg;
            WUCF { WoWoRegFieldBit }
            CMCF { WoWoRegFieldBit }
            UDRCF { WoWoRegFieldBit Option }
            EOBCF { WoWoRegFieldBit Option }
            RTOCF { WoWoRegFieldBit Option }
            CTSCF { WoWoRegFieldBit }
            LBDCF { WoWoRegFieldBit Option }
            TCBGTCF { WoWoRegFieldBit Option }
            TCCF { WoWoRegFieldBit }
            TXFECF { WoWoRegFieldBit Option }
            IDLECF { WoWoRegFieldBit }
            ORECF { WoWoRegFieldBit }
            NECF { WoWoRegFieldBit }
            FECF { WoWoRegFieldBit }
            PECF { WoWoRegFieldBit }
        }
        RDR {
            0x20 RoReg;
            RDR { RoRoRegFieldBits }
        }
        TDR {
            0x20 RwReg;
            TDR { RwRwRegFieldBits }
        }
        PRESC {
            0x20 RwReg;
            PRESCALER { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_uart {
    (
        $uart_macro_doc:expr,
        $uart_macro:ident,
        $uart_ty_doc:expr,
        $uart_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $dccipr:ident,
        $uarten:ident,
        $uartrst:ident,
        $uartsmen:ident,
        $uartsel:ident,
        $uart:ident,
        ($($eobie:ident)?),
        ($($rtoie:ident)?),
        ($($over8:ident)?),
        ($($rtoen:ident)?),
        ($($abrmod0:ident)?),
        ($($abrmod1:ident)?),
        ($($abren:ident)?),
        ($($linen:ident)?),
        ($($clken:ident)?),
        ($($cpha:ident)?),
        ($($cpol:ident)?),
        ($($lbcl:ident)?),
        ($($lbdie:ident)?),
        ($($lbdl:ident)?),
        ($($dis_nss:ident)?),
        ($($slven:ident)?),
        ($($tcbgtie:ident)?),
        ($($scarcnt:ident)?),
        ($($onebit:ident)?),
        ($($scen:ident)?),
        ($($nack:ident)?),
        ($($irlp:ident)?),
        ($($iren:ident)?),
        ($($rtor:ident)?),
        ($($abrrq:ident)?),
        ($($tcbgt:ident)?),
        ($($abrf:ident)?),
        ($($abre:ident)?),
        ($($udr:ident)?),
        ($($eobf:ident)?),
        ($($rtof:ident)?),
        ($($lbdf:ident)?),
        ($($udrcf:ident)?),
        ($($eobcf:ident)?),
        ($($rtocf:ident)?),
        ($($lbdcf:ident)?),
        ($($tcbgtcf:ident)?),
        ($($txfecf:ident)?),
    ) => {
        periph::map! {
            #[doc = $uart_macro_doc]
            pub macro $uart_macro;

            #[doc = $uart_ty_doc]
            pub struct $uart_ty;

            impl UartMap for $uart_ty {}

            drone_stm32_map_pieces::reg;
            crate;

            RCC {
                BUSENR {
                    $busenr Shared;
                    UARTEN { $uarten }
                }
                BUSRSTR {
                    $busrstr Shared;
                    UARTRST { $uartrst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    UARTSMEN { $uartsmen }
                }
                DCCIPR {
                    $dccipr Shared;
                    UARTSEL { $uartsel }
                }
            }

            UART {
                $uart;
                CR1 {
                    CR1;
                    RXFFIE { RXFFIE }
                    TXFEIE { TXFEIE }
                    FIFOEN { FIFOEN }
                    M1 { M1 }
                    EOBIE { $($eobie Option)* }
                    RTOIE { $($rtoie Option)* }
                    DEAT { DEAT }
                    DEDT { DEDT }
                    OVER8 { $($over8 Option)* }
                    CMIE { CMIE }
                    MME { MME }
                    M0 { M0 }
                    WAKE { WAKE }
                    PCE { PCE }
                    PS { PS }
                    PEIE { PEIE }
                    TXFNFIE { TXFNFIE }
                    TCIE { TCIE }
                    RXFNEIE { RXFNEIE }
                    IDLEIE { IDLEIE }
                    TE { TE }
                    RE { RE }
                    UESM { UESM }
                    UE { UE }
                }
                CR2 {
                    CR2;
                    ADD { ADD }
                    RTOEN { $($rtoen Option)* }
                    ABRMOD0 { $($abrmod0 Option)* }
                    ABRMOD1 { $($abrmod1 Option)* }
                    ABREN { $($abren Option)* }
                    MSBFIRST { MSBFIRST }
                    DATAINV { DATAINV }
                    TXINV { TXINV }
                    RXINV { RXINV }
                    SWAP { SWAP }
                    LINEN { $($linen Option)* }
                    STOP { STOP }
                    CLKEN { $($clken Option)* }
                    CPHA { $($cpha Option)* }
                    CPOL { $($cpol Option)* }
                    LBCL { $($lbcl Option)* }
                    LBDIE { $($lbdie Option)* }
                    LBDL { $($lbdl Option)* }
                    ADDM7 { ADDM7 }
                    DIS_NSS { $($dis_nss Option)* }
                    SLVEN { $($slven Option)* }
                }
                CR3 {
                    CR3;
                    TXFTCFG { TXFTCFG }
                    RXFTIE { RXFTIE }
                    RXFTCFG { RXFTCFG }
                    TCBGTIE { $($tcbgtie Option)* }
                    TXFTIE { TXFTIE }
                    WUFIE { WUFIE }
                    WUS { WUS }
                    SCARCNT { $($scarcnt Option)* }
                    DEP { DEP }
                    DEM { DEM }
                    DDRE { DDRE }
                    OVRDIS { OVRDIS }
                    ONEBIT { $($onebit Option)* }
                    CTSIE { CTSIE }
                    CTSE { CTSE }
                    RTSE { RTSE }
                    DMAT { DMAT }
                    DMAR { DMAR }
                    SCEN { $($scen Option)* }
                    NACK { $($nack Option)* }
                    HDSEL { HDSEL }
                    IRLP { $($irlp Option)* }
                    IREN { $($iren Option)* }
                    EIE { EIE }
                }
                BRR {
                    BRR;
                    BRR { BRR }
                }
                GTPR {
                    GTPR;
                    GT { GT }
                    PSC { PSC }
                }
                RTOR {
                    $(
                        $rtor Option;
                        BLEN { BLEN }
                        RTO { RTO }
                    )*
                }
                RQR {
                    RQR;
                    TXFRQ { TXFRQ }
                    RXFRQ { RXFRQ }
                    MMRQ { MMRQ }
                    SBKRQ { SBKRQ }
                    ABRRQ { $($abrrq Option)* }
                }
                ISR {
                    ISR;
                    TXFT { TXFT }
                    RXFT { RXFT }
                    TCBGT { $($tcbgt Option)* }
                    RXFF { RXFF }
                    TXFE { TXFE }
                    REACK { REACK }
                    TEACK { TEACK }
                    WUF { WUF }
                    RWU { RWU }
                    SBKF { SBKF }
                    CMF { CMF }
                    BUSY { BUSY }
                    ABRF { $($abrf Option)* }
                    ABRE { $($abre Option)* }
                    UDR { $($udr Option)* }
                    EOBF { $($eobf Option)* }
                    RTOF { $($rtof Option)* }
                    CTS { CTS }
                    CTSIF { CTSIF }
                    LBDF { $($lbdf Option)* }
                    TXE { TXE }
                    TC { TC }
                    RXNE { RXNE }
                    IDLE { IDLE }
                    ORE { ORE }
                    NE { NE }
                    FE { FE }
                    PE { PE }
                }
                ICR {
                    ICR;
                    WUCF { WUCF }
                    CMCF { CMCF }
                    UDRCF { $($udrcf Option)* }
                    EOBCF { $($eobcf Option)* }
                    RTOCF { $($rtocf Option)* }
                    CTSCF { CTSCF }
                    LBDCF { $($lbdcf Option)* }
                    TCBGTCF { $($tcbgtcf Option)* }
                    TCCF { TCCF }
                    TXFECF { $($txfecf Option)* }
                    IDLECF { IDLECF }
                    ORECF { ORECF }
                    NECF { NECF }
                    FECF { FECF }
                    PECF { PECF }
                }
                RDR {
                    RDR;
                    RDR { RDR }
                }
                TDR {
                    TDR;
                    TDR { TDR }
                }
                PRESC {
                    PRESC;
                    PRESCALER { PRESCALER }
                }
            }
        }
    };
}

map_uart! {
    "Extracts USART1 register tokens.",
    periph_usart1,
    "USART1 peripheral variant.",
    Usart1,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    D2CCIP2R,
    USART1EN,
    USART1RST,
    USART1LPEN,
    USART16SEL,
    USART1,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts USART2 register tokens.",
    periph_usart2,
    "USART2 peripheral variant.",
    Usart2,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    USART2EN,
    USART2RST,
    USART2LPEN,
    USART234578SEL,
    USART2,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts USART3 register tokens.",
    periph_usart3,
    "USART3 peripheral variant.",
    Usart3,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    USART3EN,
    USART3RST,
    USART3LPEN,
    USART234578SEL,
    USART3,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts UART4 register tokens.",
    periph_uart4,
    "UART4 peripheral variant.",
    Uart4,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    UART4EN,
    UART4RST,
    UART4LPEN,
    USART234578SEL,
    UART4,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts UART5 register tokens.",
    periph_uart5,
    "UART5 peripheral variant.",
    Uart5,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    UART5EN,
    UART5RST,
    UART5LPEN,
    USART234578SEL,
    UART5,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts USART6 register tokens.",
    periph_usart6,
    "USART6 peripheral variant.",
    Usart6,
    APB2ENR,
    APB2RSTR,
    APB2LPENR,
    D2CCIP2R,
    USART6EN,
    USART6RST,
    USART6LPEN,
    USART16SEL,
    USART6,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts UART7 register tokens.",
    periph_uart7,
    "UART7 peripheral variant.",
    Uart7,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    UART7EN,
    UART7RST,
    UART7LPEN,
    USART234578SEL,
    UART7,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts UART8 register tokens.",
    periph_uart8,
    "UART8 peripheral variant.",
    Uart8,
    APB1LENR,
    APB1LRSTR,
    APB1LLPENR,
    D2CCIP2R,
    UART8EN,
    UART8RST,
    UART8LPEN,
    USART234578SEL,
    UART8,
    (EOBIE),
    (RTOIE),
    (OVER8),
    (RTOEN),
    (ABRMOD0),
    (ABRMOD1),
    (ABREN),
    (LINEN),
    (CLKEN),
    (CPHA),
    (CPOL),
    (LBCL),
    (LBDIE),
    (LBDL),
    (DIS_NSS),
    (SLVEN),
    (TCBGTIE),
    (SCARCNT),
    (ONEBIT),
    (SCEN),
    (NACK),
    (IRLP),
    (IREN),
    (RTOR),
    (ABRRQ),
    (TCBGT),
    (ABRF),
    (ABRE),
    (UDR),
    (EOBF),
    (RTOF),
    (LBDF),
    (UDRCF),
    (EOBCF),
    (RTOCF),
    (LBDCF),
    (TCBGTCF),
    (TXFECF),
}

map_uart! {
    "Extracts LPUART1 register tokens.",
    periph_lpuart1,
    "LPUART1 peripheral variant.",
    Lpuart1,
    APB4ENR,
    APB4RSTR,
    APB4LPENR,
    D3CCIPR,
    LPUART1EN,
    LPUART1RST,
    LPUART1LPEN,
    LPUART1SEL,
    LPUART1,
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
    (),
}
