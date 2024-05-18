use crate::Result;

pub mod enums;
pub mod flags;
pub mod structs;

mod channel;
mod channel_control;
mod channel_group;
mod dsp;
mod dsp_connection;
mod geometry;
mod reverb_3d;
mod sound;
mod sound_group;
mod sync_point;
mod system;
mod system_builder;

pub fn bind(module: magnus::RModule) -> Result<()> {
    structs::bind(module)?;
    enums::bind(module)?;
    flags::bind(module)?;

    channel_control::bind(module)?;
    channel_group::bind(module)?;
    channel::bind(module)?;
    system_builder::bind(module)?;
    system::bind(module)?;
    dsp::bind(module)?;
    sound_group::bind(module)?;
    reverb_3d::bind(module)?;
    sound::bind(module)?;
    geometry::bind(module)?;
    dsp_connection::bind(module)?;
    sync_point::bind(module)?;

    Ok(())
}
