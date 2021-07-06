//! RTC peripheral patches.

use anyhow::Result;
use drone_svd::Device;

pub fn fix_1(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("APB1ENR1").new_field(|field| {
        field.name = "RTCAPBEN".to_string();
        field.description = "RTC APB clock enable".to_string();
        field.bit_offset = Some(10);
        field.bit_width = Some(1);
    });
    dev.periph("RCC").reg("APB1SMENR1").new_field(|field| {
        field.name = "RTCAPBSMEN".to_string();
        field.description = "RTC APB clock enable during Sleep and Stop modes".to_string();
        field.bit_offset = Some(10);
        field.bit_width = Some(1);
    });
    Ok(())
}

pub fn fix_2(dev: &mut Device) -> Result<()> {
    dev.periph("RCC").reg("BDCR").field("RTCSRC").name = "RTCSEL".to_string();
    dev.periph("RTC").reg("RTC_TR").name = "TR".to_string();
    dev.periph("RTC").reg("RTC_DR").name = "DR".to_string();
    dev.periph("RTC").reg("RTC_CR").name = "CR".to_string();
    dev.periph("RTC").reg("RTC_ISR").name = "ISR".to_string();
    dev.periph("RTC").reg("RTC_PRER").name = "PRER".to_string();
    dev.periph("RTC").reg("RTC_WUTR").name = "WUTR".to_string();
    dev.periph("RTC").reg("RTC_ALRMAR").name = "ALRMAR".to_string();
    dev.periph("RTC").reg("RTC_ALRMBR").name = "ALRMBR".to_string();
    dev.periph("RTC").reg("RTC_WPR").name = "WPR".to_string();
    dev.periph("RTC").reg("RTC_SSR").name = "SSR".to_string();
    dev.periph("RTC").reg("RTC_SHIFTR").name = "SHIFTR".to_string();
    dev.periph("RTC").reg("RTC_TSTR").name = "TSTR".to_string();
    dev.periph("RTC").reg("RTC_TSDR").name = "TSDR".to_string();
    dev.periph("RTC").reg("RTC_TSSSR").name = "TSSSR".to_string();
    dev.periph("RTC").reg("RTC_CALR").name = "CALR".to_string();
    dev.periph("RTC").reg("RTC_TAMPCR").name = "TAMPCR".to_string();
    dev.periph("RTC").reg("RTC_ALRMASSR").name = "ALRMASSR".to_string();
    dev.periph("RTC").reg("RTC_ALRMBSSR").name = "ALRMBSSR".to_string();
    dev.periph("RTC").reg("RTC_OR").name = "OR".to_string();
    for i in 0..=31 {
        dev.periph("RTC").reg(&format!("RTC_BKP{}R", i)).name = format!("BKP{}R", i);
    }
    Ok(())
}
