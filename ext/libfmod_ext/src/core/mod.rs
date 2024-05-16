use crate::Result;

pub mod enums;
pub mod flags;
pub mod structs;

mod channel;
mod channel_group;
mod dsp;
mod geometry;
mod reverb_3d;
mod sound;
mod sound_group;
mod system;
mod system_builder;

pub fn bind(module: magnus::RModule) -> Result<()> {
    structs::bind(module)?;
    enums::bind(module)?;
    flags::bind(module)?;

    system_builder::bind(module)?;
    system::bind(module)?;
    dsp::bind(module)?;
    channel_group::bind(module)?;
    channel::bind(module)?;
    sound_group::bind(module)?;
    reverb_3d::bind(module)?;
    sound::bind(module)?;
    geometry::bind(module)?;

    Ok(())
}
