//! DMAMUX peripheral patches.

use crate::parse_svd;
use anyhow::Result;
use drone_svd::Device;

pub fn add_dmamux1(dev: &mut Device) -> Result<()> {
    dev.add_periph(parse_svd("patch/add_dmamux.xml")?.periph("DMAMUX1").clone());
    Ok(())
}

pub fn fix_dmamux(dev: &mut Device) -> Result<()> {
    for periph_name in &["DMAMUX1", "DMAMUX2"] {
        dev.periph(periph_name).reg("RGSR").remove_field("OF");
        dev.periph(periph_name).reg("RGCFR").remove_field("COF");
        for i in 0..=7 {
            dev.periph(periph_name).reg("RGSR").new_field(|field| {
                field.name = format!("OF{}", i);
                field.description = "Trigger overrun event flag".to_string();
                field.bit_offset = Some(i);
                field.bit_width = Some(1);
            });
            dev.periph(periph_name).reg("RGCFR").new_field(|field| {
                field.name = format!("COF{}", i);
                field.description = "Clear trigger overrun event flag".to_string();
                field.bit_offset = Some(i);
                field.bit_width = Some(1);
            });
        }
        dev.periph(periph_name).reg("CSR").remove_field("SOF");
        dev.periph(periph_name).reg("CFR").remove_field("CSOF");
        for i in 0..=15 {
            dev.periph(periph_name).reg("CSR").new_field(|field| {
                field.name = format!("SOF{}", i);
                field.description = "Synchronization overrun event flag".to_string();
                field.bit_offset = Some(i);
                field.bit_width = Some(1);
            });
            dev.periph(periph_name).reg("CFR").new_field(|field| {
                field.name = format!("CSOF{}", i);
                field.description = "Clear synchroniation overrun event flag".to_string();
                field.bit_offset = Some(i);
                field.bit_width = Some(1);
            });
        }
    }
    Ok(())
}
