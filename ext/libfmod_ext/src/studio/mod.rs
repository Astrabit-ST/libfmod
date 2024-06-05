use crate::Result;
use magnus::prelude::*;

mod enums;
mod flags;
mod structs;

pub mod bank;
pub mod bus;
pub mod command_replay;
pub mod command_replay_callbacks;
pub mod event_callback;
pub mod event_description;
pub mod event_instance;
pub mod system;
pub mod system_builder;
pub mod system_callback;
pub mod vca;

pub fn bind(module: magnus::RModule) -> Result<()> {
    let module = module.define_module("Studio")?;

    enums::bind(module)?;
    flags::bind(module)?;
    structs::bind(module)?;

    system_builder::bind(module)?;
    system::bind(module)?;
    system_callback::bind(module)?;
    bank::bind(module)?;
    bus::bind(module)?;
    event_description::bind(module)?;
    event_instance::bind(module)?;
    event_callback::bind(module)?;
    command_replay::bind(module)?;
    command_replay_callbacks::bind(module)?;
    vca::bind(module)?;

    Ok(())
}
