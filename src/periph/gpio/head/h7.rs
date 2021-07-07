//! Mappings for General Purpose I/O port heads.
//!
//! For STM32H7 Series of mainstream MCUs.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO port head peripheral variant.
    pub trait GpioHeadMap {}

    /// Generic GPIO port head peripheral.
    pub struct GpioHeadPeriph;

    RCC {
        BUSENR {
            0x20 RwReg Shared;
            GPIOEN { RwRwRegFieldBit }
        }
        BUSRSTR {
            0x20 RwReg Shared;
            GPIORST { RwRwRegFieldBit }
        }
        BUSSMENR {
            0x20 RwReg Shared;
            GPIOSMEN { RwRwRegFieldBit }
        }
    }

    GPIO {
        LCKR {
            0x20 RwReg Shared;
            LCKK { RwRwRegFieldBit }
        }
    }
}

macro_rules! map_gpio_port_head {
    (
        $port_macro_doc:expr,
        $port_macro:ident,
        $port_ty_doc:expr,
        $port_ty:ident,
        $busenr:ident,
        $busrstr:ident,
        $bussmenr:ident,
        $gpio:ident,
        $gpioen:ident,
        $gpiorst:ident,
        $gpiosmen:ident,
    ) => {
        periph::map! {
            #[doc = $port_macro_doc]
            pub macro $port_macro;

            #[doc = $port_ty_doc]
            pub struct $port_ty;

            impl GpioHeadMap for $port_ty {}

            drone_stm32_map_pieces::reg;
            crate::head;

            RCC {
                BUSENR {
                    $busenr Shared;
                    GPIOEN { $gpioen }
                }
                BUSRSTR {
                    $busrstr Shared;
                    GPIORST { $gpiorst }
                }
                BUSSMENR {
                    $bussmenr Shared;
                    GPIOSMEN { $gpiosmen }
                }
            }

            GPIO {
                $gpio;
                LCKR {
                    LCKR Shared;
                    LCKK { LCKK }
                }
            }
        }
    };
}

map_gpio_port_head! {
    "Extracts GPIO port A head register tokens.",
    periph_gpio_a_head,
    "GPIO port A head peripheral variant.",
    GpioAHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOA,
    GPIOAEN,
    GPIOARST,
    GPIOALPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port B head register tokens.",
    periph_gpio_b_head,
    "GPIO port B head peripheral variant.",
    GpioBHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOB,
    GPIOBEN,
    GPIOBRST,
    GPIOBLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port C head register tokens.",
    periph_gpio_c_head,
    "GPIO port C head peripheral variant.",
    GpioCHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOC,
    GPIOCEN,
    GPIOCRST,
    GPIOCLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port D head register tokens.",
    periph_gpio_d_head,
    "GPIO port D head peripheral variant.",
    GpioDHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOD,
    GPIODEN,
    GPIODRST,
    GPIODLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port E head register tokens.",
    periph_gpio_e_head,
    "GPIO port E head peripheral variant.",
    GpioEHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOE,
    GPIOEEN,
    GPIOERST,
    GPIOELPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port F head register tokens.",
    periph_gpio_f_head,
    "GPIO port F head peripheral variant.",
    GpioFHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOF,
    GPIOFEN,
    GPIOFRST,
    GPIOFLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port G head register tokens.",
    periph_gpio_g_head,
    "GPIO port G head peripheral variant.",
    GpioGHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOG,
    GPIOGEN,
    GPIOGRST,
    GPIOGLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port H head register tokens.",
    periph_gpio_h_head,
    "GPIO port H head peripheral variant.",
    GpioHHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOH,
    GPIOHEN,
    GPIOHRST,
    GPIOHLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port I head register tokens.",
    periph_gpio_i_head,
    "GPIO port I head peripheral variant.",
    GpioIHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOI,
    GPIOIEN,
    GPIOIRST,
    GPIOILPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port J head register tokens.",
    periph_gpio_j_head,
    "GPIO port J head peripheral variant.",
    GpioJHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOJ,
    GPIOJEN,
    GPIOJRST,
    GPIOJLPEN,
}

map_gpio_port_head! {
    "Extracts GPIO port K head register tokens.",
    periph_gpio_k_head,
    "GPIO port K head peripheral variant.",
    GpioKHead,
    AHB4ENR,
    AHB4RSTR,
    AHB4LPENR,
    GPIOK,
    GPIOKEN,
    GPIOKRST,
    GPIOKLPEN,
}
