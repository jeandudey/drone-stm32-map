//! EXTI peripheral patches.

use anyhow::Result;
use drone_svd::{Device, Access};

pub fn fix_exti_1(dev: &mut Device) -> Result<()> {
    for (reg_name, field_name) in &[("IMR2", "MR39"), ("EMR2", "MR39")] {
        let mut field = dev.periph("EXTI").reg(reg_name).field(field_name).clone();
        field.name = field.name.replace("39", "40");
        field.description = field.description.replace("39", "40");
        field.bit_offset = Some(field.bit_offset.unwrap() + 1);
        dev.periph("EXTI").reg(reg_name).add_field(field);
    }
    Ok(())
}

pub fn fix_exti_2(dev: &mut Device) -> Result<()> {
    for (reg_name, field_name) in &[
        ("IMR", "MR22"),
        ("EMR", "MR22"),
        ("RTSR", "TR22"),
        ("FTSR", "TR22"),
        ("SWIER", "SWIER22"),
        ("PR", "PR22"),
    ] {
        let mut field = dev.periph("EXTI").reg(reg_name).field(field_name).clone();
        field.name = field.name.replace("22", "23");
        field.description = field.description.replace("22", "23");
        field.bit_offset = Some(field.bit_offset.unwrap() + 1);
        dev.periph("EXTI").reg(reg_name).add_field(field);
    }
    Ok(())
}

pub fn fix_exti_3(dev: &mut Device) -> Result<()> {
    dev.periph("EXTI").reg("CPUEMR3").access = Some(Access::ReadWrite);
    dev.periph("EXTI").reg("CPUIMR3").access = Some(Access::ReadWrite);
    dev.periph("EXTI").reg("CPUPR3").access = Some(Access::ReadWrite);
    dev.periph("EXTI").reg("CPUPR2").access = Some(Access::ReadWrite);
    for i in 0..=31 {
        if i == 13 {
            continue;
        }
        dev.periph("EXTI").reg("CPUIMR2").remove_field(&format!("MR{}", i));
    }
    for i in 32..=63 {
        if i == 45 {
            continue;
        }
        dev.periph("EXTI").reg("CPUIMR2").new_field(|field| {
            field.name = format!("MR{}", i);
            field.description = "CPU Interrupt Mask on Direct Event input x+32".to_string();
            field.bit_offset = Some(i - 32);
            field.bit_width = Some(1);
        });
    }
    Ok(())
}
