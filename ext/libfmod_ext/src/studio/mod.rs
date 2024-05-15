use crate::Result;
use magnus::prelude::*;

mod enums;
mod flags;
mod structs;

mod system;

pub fn bind(module: magnus::RModule) -> Result<()> {
    let module = module.define_module("Studio")?;

    enums::bind(module)?;
    flags::bind(module)?;
    structs::bind(module)?;

    system::bind(module)?;

    Ok(())
}
