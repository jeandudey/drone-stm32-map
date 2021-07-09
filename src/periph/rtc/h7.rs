//! Real-time clock.
//!
//! For STM32H7 series of MCUs.

use drone_core::periph;

periph::singular! {
    /// Extracts RTC register tokens.
    pub macro periph_rtc;

    /// RTC peripheral.
    pub struct RtcPeriph;

    drone_stm32_map_pieces::reg;
    crate;

    RCC {
        APB4ENR {
            RTCAPBEN;
        }
        APB4LPENR {
            RTCAPBLPEN;
        }
        BDCR {
            RTCEN;
            RTCSEL;
        }
    }

    RTC {
        TR;
        DR;
        CR;
        ISR;
        PRER;
        WUTR;
        ALRMAR;
        ALRMBR;
        WPR;
        SSR;
        SHIFTR;
        TSTR;
        TSDR;
        TSSSR;
        CALR;
        TAMPCR;
        ALRMASSR;
        ALRMBSSR;
        OR;
        BKP0R;
        BKP1R;
        BKP2R;
        BKP3R;
        BKP4R;
        BKP5R;
        BKP6R;
        BKP7R;
        BKP8R;
        BKP9R;
        BKP10R;
        BKP11R;
        BKP12R;
        BKP13R;
        BKP14R;
        BKP15R;
        BKP16R;
        BKP17R;
        BKP18R;
        BKP19R;
        BKP20R;
        BKP21R;
        BKP22R;
        BKP23R;
        BKP24R;
        BKP25R;
        BKP26R;
        BKP27R;
        BKP28R;
        BKP29R;
        BKP30R;
        BKP31R;
    }
}
