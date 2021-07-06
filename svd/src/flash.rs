//! FLASH peripheral patches.

use anyhow::Result;
use drone_svd::Device;

pub fn fix(dev: &mut Device) -> Result<()> {
    dev.periph("Flash").remove_reg("ACR_");
    dev.periph("Flash").remove_reg("OPTKEYR_");
    dev.periph("Flash").remove_reg("OPTCR_");
    dev.periph("Flash").remove_reg("OPTSR_CUR_");
    dev.periph("Flash").remove_reg("OPTSR_PRG_");
    dev.periph("Flash").remove_reg("OPTCCR_");
    dev.periph("Flash").reg("BOOT_CURR").name = "BOOT_CUR".to_string();
    dev.periph("Flash").reg("BOOT_PRGR").name = "BOOT_PRG".to_string();
    dev.periph("Flash").name = "FLASH".to_string();
    Ok(())
}
