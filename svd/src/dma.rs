//! DMA peripheral patches.

use anyhow::Result;
use drone_svd::{Access, Device};

pub fn fix_dma1_1(dev: &mut Device) -> Result<()> {
    for reg_name in &["S1CR", "S2CR", "S3CR", "S4CR", "S5CR", "S6CR", "S7CR"] {
        dev.periph("DMA1").reg(reg_name).remove_field("ACK");
    }
    Ok(())
}

pub fn fix_dma1_2(dev: &mut Device) -> Result<()> {
    for i in 0..=7 {
        println!("{}", i);
        dev.periph("DMA1").reg(&format!("S{}PAR", i)).field("PA").name = "PAR".to_string();
        dev.periph("DMA1").reg(&format!("S{}CR", i)).new_field(|field| {
            field.name = "TRBUFF".to_string();
            field.description = "Enable the DMA to handle bufferable transfers".to_string();
            field.bit_offset = Some(20);
            field.bit_width = Some(1);
        });
    }
    dev.periph("DMA1").reg("LIFCR").access = Some(Access::WriteOnly);
    dev.periph("DMA1").reg("HIFCR").access = Some(Access::WriteOnly);

    Ok(())
}

pub fn fix_dma2_1(dev: &mut Device) -> Result<()> {
    for reg_name in &["S1CR", "S2CR", "S3CR", "S4CR", "S5CR", "S6CR", "S7CR"] {
        dev.periph("DMA2").reg(reg_name).remove_field("ACK");
    }
    Ok(())
}

pub fn fix_dma2_2(dev: &mut Device) -> Result<()> {
    dev.periph("DMA2").reg("LIFCR").access = Some(Access::WriteOnly);
    dev.periph("DMA2").reg("HIFCR").access = Some(Access::WriteOnly);
    Ok(())
}
