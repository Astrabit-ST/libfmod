use crate::Result;
use magnus::prelude::*;

mod enums;
mod flags;
mod structs;

mod bank;
mod bus;
mod command_replay;
mod event_description;
mod event_instance;
mod system;
mod system_builder;
mod vca;

pub fn bind(module: magnus::RModule) -> Result<()> {
    let module = module.define_module("Studio")?;

    enums::bind(module)?;
    flags::bind(module)?;
    structs::bind(module)?;

    system_builder::bind(module)?;
    system::bind(module)?;
    bank::bind(module)?;
    bus::bind(module)?;
    event_description::bind(module)?;
    event_instance::bind(module)?;
    command_replay::bind(module)?;
    vca::bind(module)?;

    Ok(())
}
