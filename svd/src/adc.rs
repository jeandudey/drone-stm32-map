//! ADC peripheral patches.

use anyhow::Result;
use drone_svd::{Access, Device};

pub fn fix_adc1_1(dev: &mut Device) -> Result<()> {
    dev.periph("ADC1").reg("SMPR1").remove_field("SMPx_x");
    for i in 0..=8 {
        dev.periph("ADC1").reg("SMPR1").new_field(|field| {
            field.name = format!("SMP{}", 10 + i);
            field.description = "Channel x sampling time selection".to_string();
            field.bit_offset = Some(i * 3);
            field.bit_width = Some(3);
        });
    }
    dev.periph("ADC1").reg("SMPR2").remove_field("SMPx_x");
    for i in 0..=9 {
        dev.periph("ADC1").reg("SMPR2").new_field(|field| {
            field.name = format!("SMP{}", i);
            field.description = "Channel x sampling time selection".to_string();
            field.bit_offset = Some(i * 3);
            field.bit_width = Some(3);
        });
    }
    Ok(())
}

pub fn fix_adc_1(dev: &mut Device) -> Result<()> {
    dev.periph("ADC").reg("SMPR1").remove_field("SMPPLUS");
    Ok(())
}

pub fn fix_adc_2(dev: &mut Device, periph_name: &str) -> Result<()> {
    for i in 1..=4 {
        dev.periph(periph_name).reg(&format!("JDR{}", i)).field(&format!("JDATA{}", i)).name =
            "JDATA".to_string();
    }
    Ok(())
}

pub fn fix_adc_3(dev: &mut Device) -> Result<()> {
    for i in 2..=4 {
        let reg_name = format!("OFR{}", i);
        dev.periph("ADC3").reg(&reg_name).field("OFFSET1").name = format!("OFFSET{}", i);
        dev.periph("ADC3").reg(&reg_name).field("OFFSET1_CH").name = format!("OFFSET{}_CH", i);
    }
    dev.periph("ADC3").reg("SQR1").field("L3").name = "L".to_string();
    dev.periph("ADC3").reg("PCSEL").remove_field("PCSEL");
    for i in 0..=19 {
        dev.periph("ADC3").reg("PCSEL").new_field(|field| {
            field.name = format!("PCSEL{}", i);
            field.bit_offset = Some(i);
            field.bit_width = Some(1);
            field.description = format!("Channel {} (VINP[i]) pre selection", i);
        });
    }
    dev.periph("ADC3").reg("SMPR1").new_field(|field| {
        field.name = "SMP0".to_string();
        field.bit_offset = Some(0);
        field.bit_width = Some(3);
        field.description = "ADC channel 0 sampling time selection".to_string();
    });
    dev.periph("ADC3").reg("CFGR2").field("OSR").name = "OSVR".to_string();
    dev.periph("ADC3").reg("ISR").new_field(|field| {
        field.name = "LDORDY".to_string();
        field.bit_offset = Some(12);
        field.bit_width = Some(1);
        field.description = "ADC LDO output voltage ready bit".to_string();
        field.access = Some(Access::ReadOnly);
    });
    dev.periph("ADC3").reg("LHTR1").field("LHTR1").name = "HTR1".to_string();
    dev.periph("ADC3").reg("LHTR1").name = "HTR1".to_string();
    Ok(())
}

pub fn fix_adc_com(dev: &mut Device) -> Result<()> {
    dev.periph("C_ADC").name = "ADC_Common".to_string();
    Ok(())
}

pub fn fix_adc_com_tsen(dev: &mut Device) -> Result<()> {
    dev.periph("ADC3_Common").reg("CCR").field("VSENSEEN").name = "TSEN".to_string();
    Ok(())
}
