//! GPIO peripheral patches.

use crate::parse_svd;
use anyhow::Result;
use drone_svd::Device;

pub fn add_ascr(dev: &mut Device) -> Result<()> {
    let mut patch = parse_svd("patch/add_gpio_ascr.xml")?;
    let ascr = patch.periph("GPIO").reg("ASCR");
    dev.periph("GPIOA").add_reg(ascr.clone());
    dev.periph("GPIOB").add_reg(ascr.clone());
    dev.periph("GPIOC").add_reg(ascr.clone());
    Ok(())
}

pub fn fix(dev: &mut Device) -> Result<()> {
    for i in 0..16 {
        let field = dev.periph("GPIOA").reg("MODER").field(&format!("MODE{}", i));
        field.name = field.name.replace("MODE", "MODER");
    }
    for i in 0..16 {
        let field = dev.periph("GPIOA").reg("OSPEEDR").field(&format!("OSPEED{}", i));
        field.name = field.name.replace("OSPEED", "OSPEEDR");
    }
    for i in 0..=15 {
        let field = dev.periph("GPIOA").reg("PUPDR").field(&format!("PUPD{}", i));
        field.name = field.name.replace("PUPD", "PUPDR");
    }
    for i in 0..=15 {
        let field = dev.periph("GPIOA").reg("IDR").field(&format!("ID{}", i));
        field.name = field.name.replace("ID", "IDR");
    }
    for i in 0..=15 {
        let field = dev.periph("GPIOA").reg("ODR").field(&format!("OD{}", i));
        field.name = field.name.replace("OD", "ODR");
    }
    for i in 0..=7 {
        let field = dev.periph("GPIOA").reg("AFRL").field(&format!("AFSEL{}", i));
        field.name = field.name.replace("SEL", "RL");
    }
    for i in 8..=15 {
        let field = dev.periph("GPIOA").reg("AFRH").field(&format!("AFSEL{}", i));
        field.name = field.name.replace("SEL", "RH");
    }
    Ok(())
}
