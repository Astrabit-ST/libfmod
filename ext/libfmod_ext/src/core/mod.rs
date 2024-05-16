use crate::Result;

pub mod enums;
pub mod flags;
pub mod structs;

mod system;
mod system_builder;

pub fn bind(module: magnus::RModule) -> Result<()> {
    structs::bind(module)?;
    enums::bind(module)?;
    flags::bind(module)?;

    system_builder::bind(module)?;
    system::bind(module)?;

    Ok(())
}
