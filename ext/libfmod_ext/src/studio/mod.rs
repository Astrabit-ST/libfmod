use crate::Result;
use magnus::prelude::*;

mod enums;
mod flags;
mod structs;

mod bank;
mod bus;
mod event_description;
mod system;
mod vca;

pub fn bind(module: magnus::RModule) -> Result<()> {
    let module = module.define_module("Studio")?;

    enums::bind(module)?;
    flags::bind(module)?;
    structs::bind(module)?;

    system::bind(module)?;
    bank::bind(module)?;
    bus::bind(module)?;
    event_description::bind(module)?;
    vca::bind(module)?;

    Ok(())
}
