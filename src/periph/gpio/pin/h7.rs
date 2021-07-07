//! Mappings for General Purpose I/O port pins.
//!
//! For STM32H7 Series of mainstream MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO pin peripheral variant.
    pub trait GpioPinMap {
        /// GPIO port head peripheral variant.
        type GpioHeadMap: super::super::head::GpioHeadMap;
    }

    /// Generic GPIO pin peripheral.
    pub struct GpioPinPeriph;

    GPIO {
        MODER {
            0x20 RwReg Shared;
            MODER { RwRwRegFieldBits }
        }
        OTYPER {
            0x20 RwReg Shared;
            OT { RwRwRegFieldBit }
        }
        OSPEEDR {
            0x20 RwReg Shared;
            OSPEEDR { RwRwRegFieldBits }
        }
        PUPDR {
            0x20 RwReg Shared;
            PUPDR { RwRwRegFieldBits }
        }
        IDR {
            0x20 RoReg Shared;
            IDR { RoRoRegFieldBit }
        }
        ODR {
            0x20 RwReg Shared;
            ODR { RwRwRegFieldBit }
        }
        BSRR {
            0x20 WoReg Shared;
            BR { WoWoRegFieldBit }
            BS { WoWoRegFieldBit }
        }
        LCKR {
            0x20 RwReg Shared;
            LCK { RwRwRegFieldBit }
        }
        AFR {
            0x20 RwReg Shared;
            AFR { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_gpio_pin {
    (
        $port_ty:ident,
        $pin_macro_doc:expr,
        $pin_macro:ident,
        $pin_ty_doc:expr,
        $pin_ty:ident,
        $gpio:ident,
        $moder_ty:ident,
        $ot_ty:ident,
        $ospeedr_ty:ident,
        $pupdr_ty:ident,
        $idr_ty:ident,
        $odr_ty:ident,
        $br_ty:ident,
        $bs_ty:ident,
        $lck_ty:ident,
        $afr_path:ident,
        $afr_ty:ident,
    ) => {
        periph::map! {
            #[doc = $pin_macro_doc]
            pub macro $pin_macro;

            #[doc = $pin_ty_doc]
            pub struct $pin_ty;

            impl GpioPinMap for $pin_ty {
                type GpioHeadMap = super::super::head::$port_ty;
            }

            drone_stm32_map_pieces::reg;
            crate::pin;

            GPIO {
                $gpio;
                MODER {
                    MODER Shared;
                    MODER { $moder_ty }
                }
                OTYPER {
                    OTYPER Shared;
                    OT { $ot_ty }
                }
                OSPEEDR {
                    OSPEEDR Shared;
                    OSPEEDR { $ospeedr_ty }
                }
                PUPDR {
                    PUPDR Shared;
                    PUPDR { $pupdr_ty }
                }
                IDR {
                    IDR Shared;
                    IDR { $idr_ty }
                }
                ODR {
                    ODR Shared;
                    ODR { $odr_ty }
                }
                BSRR {
                    BSRR Shared;
                    BR { $br_ty }
                    BS { $bs_ty }
                }
                LCKR {
                    LCKR Shared;
                    LCK { $lck_ty }
                }
                AFR {
                    $afr_path Shared;
                    AFR { $afr_ty }
                }
            }
        }
    };
}

macro_rules! map_gpio_pins_lower_half {
    (
        $port_ty:ident,
        $pin0_macro_doc:expr,
        $pin0_macro:ident,
        $pin0_ty_doc:expr,
        $pin0_ty:ident,
        $pin1_macro_doc:expr,
        $pin1_macro:ident,
        $pin1_ty_doc:expr,
        $pin1_ty:ident,
        $pin2_macro_doc:expr,
        $pin2_macro:ident,
        $pin2_ty_doc:expr,
        $pin2_ty:ident,
        $pin3_macro_doc:expr,
        $pin3_macro:ident,
        $pin3_ty_doc:expr,
        $pin3_ty:ident,
        $pin4_macro_doc:expr,
        $pin4_macro:ident,
        $pin4_ty_doc:expr,
        $pin4_ty:ident,
        $pin5_macro_doc:expr,
        $pin5_macro:ident,
        $pin5_ty_doc:expr,
        $pin5_ty:ident,
        $pin6_macro_doc:expr,
        $pin6_macro:ident,
        $pin6_ty_doc:expr,
        $pin6_ty:ident,
        $pin7_macro_doc:expr,
        $pin7_macro:ident,
        $pin7_ty_doc:expr,
        $pin7_ty:ident,
        $gpio:ident,
    ) => {
        map_gpio_pin! {
            $port_ty,
            $pin0_macro_doc,
            $pin0_macro,
            $pin0_ty_doc,
            $pin0_ty,
            $gpio,
            MODER0,
            OT0,
            OSPEEDR0,
            PUPDR0,
            IDR0,
            ODR0,
            BR0,
            BS0,
            LCK0,
            AFRL,
            AFRL0,
        }
        map_gpio_pin! {
            $port_ty,
            $pin1_macro_doc,
            $pin1_macro,
            $pin1_ty_doc,
            $pin1_ty,
            $gpio,
            MODER1,
            OT1,
            OSPEEDR1,
            PUPDR1,
            IDR1,
            ODR1,
            BR1,
            BS1,
            LCK1,
            AFRL,
            AFRL1,
        }
        map_gpio_pin! {
            $port_ty,
            $pin2_macro_doc,
            $pin2_macro,
            $pin2_ty_doc,
            $pin2_ty,
            $gpio,
            MODER2,
            OT2,
            OSPEEDR2,
            PUPDR2,
            IDR2,
            ODR2,
            BR2,
            BS2,
            LCK2,
            AFRL,
            AFRL2,
        }
        map_gpio_pin! {
            $port_ty,
            $pin3_macro_doc,
            $pin3_macro,
            $pin3_ty_doc,
            $pin3_ty,
            $gpio,
            MODER3,
            OT3,
            OSPEEDR3,
            PUPDR3,
            IDR3,
            ODR3,
            BR3,
            BS3,
            LCK3,
            AFRL,
            AFRL3,
        }
        map_gpio_pin! {
            $port_ty,
            $pin4_macro_doc,
            $pin4_macro,
            $pin4_ty_doc,
            $pin4_ty,
            $gpio,
            MODER4,
            OT4,
            OSPEEDR4,
            PUPDR4,
            IDR4,
            ODR4,
            BR4,
            BS4,
            LCK4,
            AFRL,
            AFRL4,
        }
        map_gpio_pin! {
            $port_ty,
            $pin5_macro_doc,
            $pin5_macro,
            $pin5_ty_doc,
            $pin5_ty,
            $gpio,
            MODER5,
            OT5,
            OSPEEDR5,
            PUPDR5,
            IDR5,
            ODR5,
            BR5,
            BS5,
            LCK5,
            AFRL,
            AFRL5,
        }
        map_gpio_pin! {
            $port_ty,
            $pin6_macro_doc,
            $pin6_macro,
            $pin6_ty_doc,
            $pin6_ty,
            $gpio,
            MODER6,
            OT6,
            OSPEEDR6,
            PUPDR6,
            IDR6,
            ODR6,
            BR6,
            BS6,
            LCK6,
            AFRL,
            AFRL6,
        }
        map_gpio_pin! {
            $port_ty,
            $pin7_macro_doc,
            $pin7_macro,
            $pin7_ty_doc,
            $pin7_ty,
            $gpio,
            MODER7,
            OT7,
            OSPEEDR7,
            PUPDR7,
            IDR7,
            ODR7,
            BR7,
            BS7,
            LCK7,
            AFRL,
            AFRL7,
        }
    };
}

macro_rules! map_gpio_pins {
    (
        $port_ty:ident,
        $pin0_macro_doc:expr,
        $pin0_macro:ident,
        $pin0_ty_doc:expr,
        $pin0_ty:ident,
        $pin1_macro_doc:expr,
        $pin1_macro:ident,
        $pin1_ty_doc:expr,
        $pin1_ty:ident,
        $pin2_macro_doc:expr,
        $pin2_macro:ident,
        $pin2_ty_doc:expr,
        $pin2_ty:ident,
        $pin3_macro_doc:expr,
        $pin3_macro:ident,
        $pin3_ty_doc:expr,
        $pin3_ty:ident,
        $pin4_macro_doc:expr,
        $pin4_macro:ident,
        $pin4_ty_doc:expr,
        $pin4_ty:ident,
        $pin5_macro_doc:expr,
        $pin5_macro:ident,
        $pin5_ty_doc:expr,
        $pin5_ty:ident,
        $pin6_macro_doc:expr,
        $pin6_macro:ident,
        $pin6_ty_doc:expr,
        $pin6_ty:ident,
        $pin7_macro_doc:expr,
        $pin7_macro:ident,
        $pin7_ty_doc:expr,
        $pin7_ty:ident,
        $pin8_macro_doc:expr,
        $pin8_macro:ident,
        $pin8_ty_doc:expr,
        $pin8_ty:ident,
        $pin9_macro_doc:expr,
        $pin9_macro:ident,
        $pin9_ty_doc:expr,
        $pin9_ty:ident,
        $pin10_macro_doc:expr,
        $pin10_macro:ident,
        $pin10_ty_doc:expr,
        $pin10_ty:ident,
        $pin11_macro_doc:expr,
        $pin11_macro:ident,
        $pin11_ty_doc:expr,
        $pin11_ty:ident,
        $pin12_macro_doc:expr,
        $pin12_macro:ident,
        $pin12_ty_doc:expr,
        $pin12_ty:ident,
        $pin13_macro_doc:expr,
        $pin13_macro:ident,
        $pin13_ty_doc:expr,
        $pin13_ty:ident,
        $pin14_macro_doc:expr,
        $pin14_macro:ident,
        $pin14_ty_doc:expr,
        $pin14_ty:ident,
        $pin15_macro_doc:expr,
        $pin15_macro:ident,
        $pin15_ty_doc:expr,
        $pin15_ty:ident,
        $gpio:ident,
    ) => {
        map_gpio_pins_lower_half! {
            $port_ty,
            $pin0_macro_doc,
            $pin0_macro,
            $pin0_ty_doc,
            $pin0_ty,
            $pin1_macro_doc,
            $pin1_macro,
            $pin1_ty_doc,
            $pin1_ty,
            $pin2_macro_doc,
            $pin2_macro,
            $pin2_ty_doc,
            $pin2_ty,
            $pin3_macro_doc,
            $pin3_macro,
            $pin3_ty_doc,
            $pin3_ty,
            $pin4_macro_doc,
            $pin4_macro,
            $pin4_ty_doc,
            $pin4_ty,
            $pin5_macro_doc,
            $pin5_macro,
            $pin5_ty_doc,
            $pin5_ty,
            $pin6_macro_doc,
            $pin6_macro,
            $pin6_ty_doc,
            $pin6_ty,
            $pin7_macro_doc,
            $pin7_macro,
            $pin7_ty_doc,
            $pin7_ty,
            $gpio,
        }
        map_gpio_pin! {
            $port_ty,
            $pin8_macro_doc,
            $pin8_macro,
            $pin8_ty_doc,
            $pin8_ty,
            $gpio,
            MODER8,
            OT8,
            OSPEEDR8,
            PUPDR8,
            IDR8,
            ODR8,
            BR8,
            BS8,
            LCK8,
            AFRH,
            AFRH8,
        }
        map_gpio_pin! {
            $port_ty,
            $pin9_macro_doc,
            $pin9_macro,
            $pin9_ty_doc,
            $pin9_ty,
            $gpio,
            MODER9,
            OT9,
            OSPEEDR9,
            PUPDR9,
            IDR9,
            ODR9,
            BR9,
            BS9,
            LCK9,
            AFRH,
            AFRH9,
        }
        map_gpio_pin! {
            $port_ty,
            $pin10_macro_doc,
            $pin10_macro,
            $pin10_ty_doc,
            $pin10_ty,
            $gpio,
            MODER10,
            OT10,
            OSPEEDR10,
            PUPDR10,
            IDR10,
            ODR10,
            BR10,
            BS10,
            LCK10,
            AFRH,
            AFRH10,
        }
        map_gpio_pin! {
            $port_ty,
            $pin11_macro_doc,
            $pin11_macro,
            $pin11_ty_doc,
            $pin11_ty,
            $gpio,
            MODER11,
            OT11,
            OSPEEDR11,
            PUPDR11,
            IDR11,
            ODR11,
            BR11,
            BS11,
            LCK11,
            AFRH,
            AFRH11,
        }
        map_gpio_pin! {
            $port_ty,
            $pin12_macro_doc,
            $pin12_macro,
            $pin12_ty_doc,
            $pin12_ty,
            $gpio,
            MODER12,
            OT12,
            OSPEEDR12,
            PUPDR12,
            IDR12,
            ODR12,
            BR12,
            BS12,
            LCK12,
            AFRH,
            AFRH12,
        }
        map_gpio_pin! {
            $port_ty,
            $pin13_macro_doc,
            $pin13_macro,
            $pin13_ty_doc,
            $pin13_ty,
            $gpio,
            MODER13,
            OT13,
            OSPEEDR13,
            PUPDR13,
            IDR13,
            ODR13,
            BR13,
            BS13,
            LCK13,
            AFRH,
            AFRH13,
        }
        map_gpio_pin! {
            $port_ty,
            $pin14_macro_doc,
            $pin14_macro,
            $pin14_ty_doc,
            $pin14_ty,
            $gpio,
            MODER14,
            OT14,
            OSPEEDR14,
            PUPDR14,
            IDR14,
            ODR14,
            BR14,
            BS14,
            LCK14,
            AFRH,
            AFRH14,
        }
        map_gpio_pin! {
            $port_ty,
            $pin15_macro_doc,
            $pin15_macro,
            $pin15_ty_doc,
            $pin15_ty,
            $gpio,
            MODER15,
            OT15,
            OSPEEDR15,
            PUPDR15,
            IDR15,
            ODR15,
            BR15,
            BS15,
            LCK15,
            AFRH,
            AFRH15,
        }
    };
}

map_gpio_pins! {
    GpioAHead,
    "Extracts GPIO port A pin 0 register tokens.",
    periph_gpio_a0,
    "GPIO port A pin 0 peripheral variant.",
    GpioA0,
    "Extracts GPIO port A pin 1 register tokens.",
    periph_gpio_a1,
    "GPIO port A pin 1 peripheral variant.",
    GpioA1,
    "Extracts GPIO port A pin 2 register tokens.",
    periph_gpio_a2,
    "GPIO port A pin 2 peripheral variant.",
    GpioA2,
    "Extracts GPIO port A pin 3 register tokens.",
    periph_gpio_a3,
    "GPIO port A pin 3 peripheral variant.",
    GpioA3,
    "Extracts GPIO port A pin 4 register tokens.",
    periph_gpio_a4,
    "GPIO port A pin 4 peripheral variant.",
    GpioA4,
    "Extracts GPIO port A pin 5 register tokens.",
    periph_gpio_a5,
    "GPIO port A pin 5 peripheral variant.",
    GpioA5,
    "Extracts GPIO port A pin 6 register tokens.",
    periph_gpio_a6,
    "GPIO port A pin 6 peripheral variant.",
    GpioA6,
    "Extracts GPIO port A pin 7 register tokens.",
    periph_gpio_a7,
    "GPIO port A pin 7 peripheral variant.",
    GpioA7,
    "Extracts GPIO port A pin 8 register tokens.",
    periph_gpio_a8,
    "GPIO port A pin 8 peripheral variant.",
    GpioA8,
    "Extracts GPIO port A pin 9 register tokens.",
    periph_gpio_a9,
    "GPIO port A pin 9 peripheral variant.",
    GpioA9,
    "Extracts GPIO port A pin 10 register tokens.",
    periph_gpio_a10,
    "GPIO port A pin 10 peripheral variant.",
    GpioA10,
    "Extracts GPIO port A pin 11 register tokens.",
    periph_gpio_a11,
    "GPIO port A pin 11 peripheral variant.",
    GpioA11,
    "Extracts GPIO port A pin 12 register tokens.",
    periph_gpio_a12,
    "GPIO port A pin 12 peripheral variant.",
    GpioA12,
    "Extracts GPIO port A pin 13 register tokens.",
    periph_gpio_a13,
    "GPIO port A pin 13 peripheral variant.",
    GpioA13,
    "Extracts GPIO port A pin 14 register tokens.",
    periph_gpio_a14,
    "GPIO port A pin 14 peripheral variant.",
    GpioA14,
    "Extracts GPIO port A pin 15 register tokens.",
    periph_gpio_a15,
    "GPIO port A pin 15 peripheral variant.",
    GpioA15,
    GPIOA,
}

map_gpio_pins! {
    GpioBHead,
    "Extracts GPIO port B pin 0 register tokens.",
    periph_gpio_b0,
    "GPIO port B pin 0 peripheral variant.",
    GpioB0,
    "Extracts GPIO port B pin 1 register tokens.",
    periph_gpio_b1,
    "GPIO port B pin 1 peripheral variant.",
    GpioB1,
    "Extracts GPIO port B pin 2 register tokens.",
    periph_gpio_b2,
    "GPIO port B pin 2 peripheral variant.",
    GpioB2,
    "Extracts GPIO port B pin 3 register tokens.",
    periph_gpio_b3,
    "GPIO port B pin 3 peripheral variant.",
    GpioB3,
    "Extracts GPIO port B pin 4 register tokens.",
    periph_gpio_b4,
    "GPIO port B pin 4 peripheral variant.",
    GpioB4,
    "Extracts GPIO port B pin 5 register tokens.",
    periph_gpio_b5,
    "GPIO port B pin 5 peripheral variant.",
    GpioB5,
    "Extracts GPIO port B pin 6 register tokens.",
    periph_gpio_b6,
    "GPIO port B pin 6 peripheral variant.",
    GpioB6,
    "Extracts GPIO port B pin 7 register tokens.",
    periph_gpio_b7,
    "GPIO port B pin 7 peripheral variant.",
    GpioB7,
    "Extracts GPIO port B pin 8 register tokens.",
    periph_gpio_b8,
    "GPIO port B pin 8 peripheral variant.",
    GpioB8,
    "Extracts GPIO port B pin 9 register tokens.",
    periph_gpio_b9,
    "GPIO port B pin 9 peripheral variant.",
    GpioB9,
    "Extracts GPIO port B pin 10 register tokens.",
    periph_gpio_b10,
    "GPIO port B pin 10 peripheral variant.",
    GpioB10,
    "Extracts GPIO port B pin 11 register tokens.",
    periph_gpio_b11,
    "GPIO port B pin 11 peripheral variant.",
    GpioB11,
    "Extracts GPIO port B pin 12 register tokens.",
    periph_gpio_b12,
    "GPIO port B pin 12 peripheral variant.",
    GpioB12,
    "Extracts GPIO port B pin 13 register tokens.",
    periph_gpio_b13,
    "GPIO port B pin 13 peripheral variant.",
    GpioB13,
    "Extracts GPIO port B pin 14 register tokens.",
    periph_gpio_b14,
    "GPIO port B pin 14 peripheral variant.",
    GpioB14,
    "Extracts GPIO port B pin 15 register tokens.",
    periph_gpio_b15,
    "GPIO port B pin 15 peripheral variant.",
    GpioB15,
    GPIOB,
}

map_gpio_pins! {
    GpioCHead,
    "Extracts GPIO port C pin 0 register tokens.",
    periph_gpio_c0,
    "GPIO port C pin 0 peripheral variant.",
    GpioC0,
    "Extracts GPIO port C pin 1 register tokens.",
    periph_gpio_c1,
    "GPIO port C pin 1 peripheral variant.",
    GpioC1,
    "Extracts GPIO port C pin 2 register tokens.",
    periph_gpio_c2,
    "GPIO port C pin 2 peripheral variant.",
    GpioC2,
    "Extracts GPIO port C pin 3 register tokens.",
    periph_gpio_c3,
    "GPIO port C pin 3 peripheral variant.",
    GpioC3,
    "Extracts GPIO port C pin 4 register tokens.",
    periph_gpio_c4,
    "GPIO port C pin 4 peripheral variant.",
    GpioC4,
    "Extracts GPIO port C pin 5 register tokens.",
    periph_gpio_c5,
    "GPIO port C pin 5 peripheral variant.",
    GpioC5,
    "Extracts GPIO port C pin 6 register tokens.",
    periph_gpio_c6,
    "GPIO port C pin 6 peripheral variant.",
    GpioC6,
    "Extracts GPIO port C pin 7 register tokens.",
    periph_gpio_c7,
    "GPIO port C pin 7 peripheral variant.",
    GpioC7,
    "Extracts GPIO port C pin 8 register tokens.",
    periph_gpio_c8,
    "GPIO port C pin 8 peripheral variant.",
    GpioC8,
    "Extracts GPIO port C pin 9 register tokens.",
    periph_gpio_c9,
    "GPIO port C pin 9 peripheral variant.",
    GpioC9,
    "Extracts GPIO port C pin 10 register tokens.",
    periph_gpio_c10,
    "GPIO port C pin 10 peripheral variant.",
    GpioC10,
    "Extracts GPIO port C pin 11 register tokens.",
    periph_gpio_c11,
    "GPIO port C pin 11 peripheral variant.",
    GpioC11,
    "Extracts GPIO port C pin 12 register tokens.",
    periph_gpio_c12,
    "GPIO port C pin 12 peripheral variant.",
    GpioC12,
    "Extracts GPIO port C pin 13 register tokens.",
    periph_gpio_c13,
    "GPIO port C pin 13 peripheral variant.",
    GpioC13,
    "Extracts GPIO port C pin 14 register tokens.",
    periph_gpio_c14,
    "GPIO port C pin 14 peripheral variant.",
    GpioC14,
    "Extracts GPIO port C pin 15 register tokens.",
    periph_gpio_c15,
    "GPIO port C pin 15 peripheral variant.",
    GpioC15,
    GPIOC,
}

map_gpio_pins! {
    GpioDHead,
    "Extracts GPIO port D pin 0 register tokens.",
    periph_gpio_d0,
    "GPIO port D pin 0 peripheral variant.",
    GpioD0,
    "Extracts GPIO port D pin 1 register tokens.",
    periph_gpio_d1,
    "GPIO port D pin 1 peripheral variant.",
    GpioD1,
    "Extracts GPIO port D pin 2 register tokens.",
    periph_gpio_d2,
    "GPIO port D pin 2 peripheral variant.",
    GpioD2,
    "Extracts GPIO port D pin 3 register tokens.",
    periph_gpio_d3,
    "GPIO port D pin 3 peripheral variant.",
    GpioD3,
    "Extracts GPIO port D pin 4 register tokens.",
    periph_gpio_d4,
    "GPIO port D pin 4 peripheral variant.",
    GpioD4,
    "Extracts GPIO port D pin 5 register tokens.",
    periph_gpio_d5,
    "GPIO port D pin 5 peripheral variant.",
    GpioD5,
    "Extracts GPIO port D pin 6 register tokens.",
    periph_gpio_d6,
    "GPIO port D pin 6 peripheral variant.",
    GpioD6,
    "Extracts GPIO port D pin 7 register tokens.",
    periph_gpio_d7,
    "GPIO port D pin 7 peripheral variant.",
    GpioD7,
    "Extracts GPIO port D pin 8 register tokens.",
    periph_gpio_d8,
    "GPIO port D pin 8 peripheral variant.",
    GpioD8,
    "Extracts GPIO port D pin 9 register tokens.",
    periph_gpio_d9,
    "GPIO port D pin 9 peripheral variant.",
    GpioD9,
    "Extracts GPIO port D pin 10 register tokens.",
    periph_gpio_d10,
    "GPIO port D pin 10 peripheral variant.",
    GpioD10,
    "Extracts GPIO port D pin 11 register tokens.",
    periph_gpio_d11,
    "GPIO port D pin 11 peripheral variant.",
    GpioD11,
    "Extracts GPIO port D pin 12 register tokens.",
    periph_gpio_d12,
    "GPIO port D pin 12 peripheral variant.",
    GpioD12,
    "Extracts GPIO port D pin 13 register tokens.",
    periph_gpio_d13,
    "GPIO port D pin 13 peripheral variant.",
    GpioD13,
    "Extracts GPIO port D pin 14 register tokens.",
    periph_gpio_d14,
    "GPIO port D pin 14 peripheral variant.",
    GpioD14,
    "Extracts GPIO port D pin 15 register tokens.",
    periph_gpio_d15,
    "GPIO port D pin 15 peripheral variant.",
    GpioD15,
    GPIOD,
}

map_gpio_pins! {
    GpioEHead,
    "Extracts GPIO port E pin 0 register tokens.",
    periph_gpio_e0,
    "GPIO port E pin 0 peripheral variant.",
    GpioE0,
    "Extracts GPIO port E pin 1 register tokens.",
    periph_gpio_e1,
    "GPIO port E pin 1 peripheral variant.",
    GpioE1,
    "Extracts GPIO port E pin 2 register tokens.",
    periph_gpio_e2,
    "GPIO port E pin 2 peripheral variant.",
    GpioE2,
    "Extracts GPIO port E pin 3 register tokens.",
    periph_gpio_e3,
    "GPIO port E pin 3 peripheral variant.",
    GpioE3,
    "Extracts GPIO port E pin 4 register tokens.",
    periph_gpio_e4,
    "GPIO port E pin 4 peripheral variant.",
    GpioE4,
    "Extracts GPIO port E pin 5 register tokens.",
    periph_gpio_e5,
    "GPIO port E pin 5 peripheral variant.",
    GpioE5,
    "Extracts GPIO port E pin 6 register tokens.",
    periph_gpio_e6,
    "GPIO port E pin 6 peripheral variant.",
    GpioE6,
    "Extracts GPIO port E pin 7 register tokens.",
    periph_gpio_e7,
    "GPIO port E pin 7 peripheral variant.",
    GpioE7,
    "Extracts GPIO port E pin 8 register tokens.",
    periph_gpio_e8,
    "GPIO port E pin 8 peripheral variant.",
    GpioE8,
    "Extracts GPIO port E pin 9 register tokens.",
    periph_gpio_e9,
    "GPIO port E pin 9 peripheral variant.",
    GpioE9,
    "Extracts GPIO port E pin 10 register tokens.",
    periph_gpio_e10,
    "GPIO port E pin 10 peripheral variant.",
    GpioE10,
    "Extracts GPIO port E pin 11 register tokens.",
    periph_gpio_e11,
    "GPIO port E pin 11 peripheral variant.",
    GpioE11,
    "Extracts GPIO port E pin 12 register tokens.",
    periph_gpio_e12,
    "GPIO port E pin 12 peripheral variant.",
    GpioE12,
    "Extracts GPIO port E pin 13 register tokens.",
    periph_gpio_e13,
    "GPIO port E pin 13 peripheral variant.",
    GpioE13,
    "Extracts GPIO port E pin 14 register tokens.",
    periph_gpio_e14,
    "GPIO port E pin 14 peripheral variant.",
    GpioE14,
    "Extracts GPIO port E pin 15 register tokens.",
    periph_gpio_e15,
    "GPIO port E pin 15 peripheral variant.",
    GpioE15,
    GPIOE,
}

map_gpio_pins! {
    GpioFHead,
    "Extracts GPIO port F pin 0 register tokens.",
    periph_gpio_f0,
    "GPIO port F pin 0 peripheral variant.",
    GpioF0,
    "Extracts GPIO port F pin 1 register tokens.",
    periph_gpio_f1,
    "GPIO port F pin 1 peripheral variant.",
    GpioF1,
    "Extracts GPIO port F pin 2 register tokens.",
    periph_gpio_f2,
    "GPIO port F pin 2 peripheral variant.",
    GpioF2,
    "Extracts GPIO port F pin 3 register tokens.",
    periph_gpio_f3,
    "GPIO port F pin 3 peripheral variant.",
    GpioF3,
    "Extracts GPIO port F pin 4 register tokens.",
    periph_gpio_f4,
    "GPIO port F pin 4 peripheral variant.",
    GpioF4,
    "Extracts GPIO port F pin 5 register tokens.",
    periph_gpio_f5,
    "GPIO port F pin 5 peripheral variant.",
    GpioF5,
    "Extracts GPIO port F pin 6 register tokens.",
    periph_gpio_f6,
    "GPIO port F pin 6 peripheral variant.",
    GpioF6,
    "Extracts GPIO port F pin 7 register tokens.",
    periph_gpio_f7,
    "GPIO port F pin 7 peripheral variant.",
    GpioF7,
    "Extracts GPIO port F pin 8 register tokens.",
    periph_gpio_f8,
    "GPIO port F pin 8 peripheral variant.",
    GpioF8,
    "Extracts GPIO port F pin 9 register tokens.",
    periph_gpio_f9,
    "GPIO port F pin 9 peripheral variant.",
    GpioF9,
    "Extracts GPIO port F pin 10 register tokens.",
    periph_gpio_f10,
    "GPIO port F pin 10 peripheral variant.",
    GpioF10,
    "Extracts GPIO port F pin 11 register tokens.",
    periph_gpio_f11,
    "GPIO port F pin 11 peripheral variant.",
    GpioF11,
    "Extracts GPIO port F pin 12 register tokens.",
    periph_gpio_f12,
    "GPIO port F pin 12 peripheral variant.",
    GpioF12,
    "Extracts GPIO port F pin 13 register tokens.",
    periph_gpio_f13,
    "GPIO port F pin 13 peripheral variant.",
    GpioF13,
    "Extracts GPIO port F pin 14 register tokens.",
    periph_gpio_f14,
    "GPIO port F pin 14 peripheral variant.",
    GpioF14,
    "Extracts GPIO port F pin 15 register tokens.",
    periph_gpio_f15,
    "GPIO port F pin 15 peripheral variant.",
    GpioF15,
    GPIOF,
}

map_gpio_pins! {
    GpioGHead,
    "Extracts GPIO port G pin 0 register tokens.",
    periph_gpio_g0,
    "GPIO port G pin 0 peripheral variant.",
    GpioG0,
    "Extracts GPIO port G pin 1 register tokens.",
    periph_gpio_g1,
    "GPIO port G pin 1 peripheral variant.",
    GpioG1,
    "Extracts GPIO port G pin 2 register tokens.",
    periph_gpio_g2,
    "GPIO port G pin 2 peripheral variant.",
    GpioG2,
    "Extracts GPIO port G pin 3 register tokens.",
    periph_gpio_g3,
    "GPIO port G pin 3 peripheral variant.",
    GpioG3,
    "Extracts GPIO port G pin 4 register tokens.",
    periph_gpio_g4,
    "GPIO port G pin 4 peripheral variant.",
    GpioG4,
    "Extracts GPIO port G pin 5 register tokens.",
    periph_gpio_g5,
    "GPIO port G pin 5 peripheral variant.",
    GpioG5,
    "Extracts GPIO port G pin 6 register tokens.",
    periph_gpio_g6,
    "GPIO port G pin 6 peripheral variant.",
    GpioG6,
    "Extracts GPIO port G pin 7 register tokens.",
    periph_gpio_g7,
    "GPIO port G pin 7 peripheral variant.",
    GpioG7,
    "Extracts GPIO port G pin 8 register tokens.",
    periph_gpio_g8,
    "GPIO port G pin 8 peripheral variant.",
    GpioG8,
    "Extracts GPIO port G pin 9 register tokens.",
    periph_gpio_g9,
    "GPIO port G pin 9 peripheral variant.",
    GpioG9,
    "Extracts GPIO port G pin 10 register tokens.",
    periph_gpio_g10,
    "GPIO port G pin 10 peripheral variant.",
    GpioG10,
    "Extracts GPIO port G pin 11 register tokens.",
    periph_gpio_g11,
    "GPIO port G pin 11 peripheral variant.",
    GpioG11,
    "Extracts GPIO port G pin 12 register tokens.",
    periph_gpio_g12,
    "GPIO port G pin 12 peripheral variant.",
    GpioG12,
    "Extracts GPIO port G pin 13 register tokens.",
    periph_gpio_g13,
    "GPIO port G pin 13 peripheral variant.",
    GpioG13,
    "Extracts GPIO port G pin 14 register tokens.",
    periph_gpio_g14,
    "GPIO port G pin 14 peripheral variant.",
    GpioG14,
    "Extracts GPIO port G pin 15 register tokens.",
    periph_gpio_g15,
    "GPIO port G pin 15 peripheral variant.",
    GpioG15,
    GPIOG,
}

map_gpio_pins! {
    GpioHHead,
    "Extracts GPIO port H pin 0 register tokens.",
    periph_gpio_h0,
    "GPIO port H pin 0 peripheral variant.",
    GpioH0,
    "Extracts GPIO port H pin 1 register tokens.",
    periph_gpio_h1,
    "GPIO port H pin 1 peripheral variant.",
    GpioH1,
    "Extracts GPIO port H pin 2 register tokens.",
    periph_gpio_h2,
    "GPIO port H pin 2 peripheral variant.",
    GpioH2,
    "Extracts GPIO port H pin 3 register tokens.",
    periph_gpio_h3,
    "GPIO port H pin 3 peripheral variant.",
    GpioH3,
    "Extracts GPIO port H pin 4 register tokens.",
    periph_gpio_h4,
    "GPIO port H pin 4 peripheral variant.",
    GpioH4,
    "Extracts GPIO port H pin 5 register tokens.",
    periph_gpio_h5,
    "GPIO port H pin 5 peripheral variant.",
    GpioH5,
    "Extracts GPIO port H pin 6 register tokens.",
    periph_gpio_h6,
    "GPIO port H pin 6 peripheral variant.",
    GpioH6,
    "Extracts GPIO port H pin 7 register tokens.",
    periph_gpio_h7,
    "GPIO port H pin 7 peripheral variant.",
    GpioH7,
    "Extracts GPIO port H pin 8 register tokens.",
    periph_gpio_h8,
    "GPIO port H pin 8 peripheral variant.",
    GpioH8,
    "Extracts GPIO port H pin 9 register tokens.",
    periph_gpio_h9,
    "GPIO port H pin 9 peripheral variant.",
    GpioH9,
    "Extracts GPIO port H pin 10 register tokens.",
    periph_gpio_h10,
    "GPIO port H pin 10 peripheral variant.",
    GpioH10,
    "Extracts GPIO port H pin 11 register tokens.",
    periph_gpio_h11,
    "GPIO port H pin 11 peripheral variant.",
    GpioH11,
    "Extracts GPIO port H pin 12 register tokens.",
    periph_gpio_h12,
    "GPIO port H pin 12 peripheral variant.",
    GpioH12,
    "Extracts GPIO port H pin 13 register tokens.",
    periph_gpio_h13,
    "GPIO port H pin 13 peripheral variant.",
    GpioH13,
    "Extracts GPIO port H pin 14 register tokens.",
    periph_gpio_h14,
    "GPIO port H pin 14 peripheral variant.",
    GpioH14,
    "Extracts GPIO port H pin 15 register tokens.",
    periph_gpio_h15,
    "GPIO port H pin 15 peripheral variant.",
    GpioH15,
    GPIOH,
}

map_gpio_pins! {
    GpioIHead,
    "Extracts GPIO port I pin 0 register tokens.",
    periph_gpio_i0,
    "GPIO port I pin 0 peripheral variant.",
    GpioI0,
    "Extracts GPIO port I pin 1 register tokens.",
    periph_gpio_i1,
    "GPIO port I pin 1 peripheral variant.",
    GpioI1,
    "Extracts GPIO port I pin 2 register tokens.",
    periph_gpio_i2,
    "GPIO port I pin 2 peripheral variant.",
    GpioI2,
    "Extracts GPIO port I pin 3 register tokens.",
    periph_gpio_i3,
    "GPIO port I pin 3 peripheral variant.",
    GpioI3,
    "Extracts GPIO port I pin 4 register tokens.",
    periph_gpio_i4,
    "GPIO port I pin 4 peripheral variant.",
    GpioI4,
    "Extracts GPIO port I pin 5 register tokens.",
    periph_gpio_i5,
    "GPIO port I pin 5 peripheral variant.",
    GpioI5,
    "Extracts GPIO port I pin 6 register tokens.",
    periph_gpio_i6,
    "GPIO port I pin 6 peripheral variant.",
    GpioI6,
    "Extracts GPIO port I pin 7 register tokens.",
    periph_gpio_i7,
    "GPIO port I pin 7 peripheral variant.",
    GpioI7,
    "Extracts GPIO port I pin 8 register tokens.",
    periph_gpio_i8,
    "GPIO port I pin 8 peripheral variant.",
    GpioI8,
    "Extracts GPIO port I pin 9 register tokens.",
    periph_gpio_i9,
    "GPIO port I pin 9 peripheral variant.",
    GpioI9,
    "Extracts GPIO port I pin 10 register tokens.",
    periph_gpio_i10,
    "GPIO port I pin 10 peripheral variant.",
    GpioI10,
    "Extracts GPIO port I pin 11 register tokens.",
    periph_gpio_i11,
    "GPIO port I pin 11 peripheral variant.",
    GpioI11,
    "Extracts GPIO port I pin 12 register tokens.",
    periph_gpio_i12,
    "GPIO port I pin 12 peripheral variant.",
    GpioI12,
    "Extracts GPIO port I pin 13 register tokens.",
    periph_gpio_i13,
    "GPIO port I pin 13 peripheral variant.",
    GpioI13,
    "Extracts GPIO port I pin 14 register tokens.",
    periph_gpio_i14,
    "GPIO port I pin 14 peripheral variant.",
    GpioI14,
    "Extracts GPIO port I pin 15 register tokens.",
    periph_gpio_i15,
    "GPIO port I pin 15 peripheral variant.",
    GpioI15,
    GPIOI,
}

map_gpio_pins! {
    GpioJHead,
    "Extracts GPIO port J pin 0 register tokens.",
    periph_gpio_j0,
    "GPIO port J pin 0 peripheral variant.",
    GpioJ0,
    "Extracts GPIO port J pin 1 register tokens.",
    periph_gpio_j1,
    "GPIO port J pin 1 peripheral variant.",
    GpioJ1,
    "Extracts GPIO port J pin 2 register tokens.",
    periph_gpio_j2,
    "GPIO port J pin 2 peripheral variant.",
    GpioJ2,
    "Extracts GPIO port J pin 3 register tokens.",
    periph_gpio_j3,
    "GPIO port J pin 3 peripheral variant.",
    GpioJ3,
    "Extracts GPIO port J pin 4 register tokens.",
    periph_gpio_j4,
    "GPIO port J pin 4 peripheral variant.",
    GpioJ4,
    "Extracts GPIO port J pin 5 register tokens.",
    periph_gpio_j5,
    "GPIO port J pin 5 peripheral variant.",
    GpioJ5,
    "Extracts GPIO port J pin 6 register tokens.",
    periph_gpio_j6,
    "GPIO port J pin 6 peripheral variant.",
    GpioJ6,
    "Extracts GPIO port J pin 7 register tokens.",
    periph_gpio_j7,
    "GPIO port J pin 7 peripheral variant.",
    GpioJ7,
    "Extracts GPIO port J pin 8 register tokens.",
    periph_gpio_j8,
    "GPIO port J pin 8 peripheral variant.",
    GpioJ8,
    "Extracts GPIO port J pin 9 register tokens.",
    periph_gpio_j9,
    "GPIO port J pin 9 peripheral variant.",
    GpioJ9,
    "Extracts GPIO port J pin 10 register tokens.",
    periph_gpio_j10,
    "GPIO port J pin 10 peripheral variant.",
    GpioJ10,
    "Extracts GPIO port J pin 11 register tokens.",
    periph_gpio_j11,
    "GPIO port J pin 11 peripheral variant.",
    GpioJ11,
    "Extracts GPIO port J pin 12 register tokens.",
    periph_gpio_j12,
    "GPIO port J pin 12 peripheral variant.",
    GpioJ12,
    "Extracts GPIO port J pin 13 register tokens.",
    periph_gpio_j13,
    "GPIO port J pin 13 peripheral variant.",
    GpioJ13,
    "Extracts GPIO port J pin 14 register tokens.",
    periph_gpio_j14,
    "GPIO port J pin 14 peripheral variant.",
    GpioJ14,
    "Extracts GPIO port J pin 15 register tokens.",
    periph_gpio_j15,
    "GPIO port J pin 15 peripheral variant.",
    GpioJ15,
    GPIOJ,
}

map_gpio_pins_lower_half! {
    GpioKHead,
    "Extracts GPIO port K pin 0 register tokens.",
    periph_gpio_k0,
    "GPIO port K pin 0 peripheral variant.",
    GpioK0,
    "Extracts GPIO port K pin 1 register tokens.",
    periph_gpio_k1,
    "GPIO port K pin 1 peripheral variant.",
    GpioK1,
    "Extracts GPIO port K pin 2 register tokens.",
    periph_gpio_k2,
    "GPIO port K pin 2 peripheral variant.",
    GpioK2,
    "Extracts GPIO port K pin 3 register tokens.",
    periph_gpio_k3,
    "GPIO port K pin 3 peripheral variant.",
    GpioK3,
    "Extracts GPIO port K pin 4 register tokens.",
    periph_gpio_k4,
    "GPIO port K pin 4 peripheral variant.",
    GpioK4,
    "Extracts GPIO port K pin 5 register tokens.",
    periph_gpio_k5,
    "GPIO port K pin 5 peripheral variant.",
    GpioK5,
    "Extracts GPIO port K pin 6 register tokens.",
    periph_gpio_k6,
    "GPIO port K pin 6 peripheral variant.",
    GpioK6,
    "Extracts GPIO port K pin 7 register tokens.",
    periph_gpio_k7,
    "GPIO port K pin 7 peripheral variant.",
    GpioK7,
    GPIOK,
}
