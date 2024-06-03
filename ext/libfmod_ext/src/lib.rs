#![warn(rust_2018_idioms)]
use magnus::prelude::*;

type Result<T> = std::result::Result<T, magnus::Error>;

mod callback;
mod core;
mod error;
mod extern_struct_storage;
mod studio;
mod thread;

mod macros;
pub use macros::*;

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<()> {
    let module = ruby.define_module("FMOD")?;

    module.const_set("VERSION", fmod::VERSION)?;
    module.const_set("MAX_LISTENERS", fmod::MAX_LISTENERS)?;
    module.const_set("MAX_SYSTEMS", fmod::MAX_SYSTEMS)?;
    module.const_set("MAX_CHANNEL_WIDTH", fmod::MAX_CHANNEL_WIDTH)?;

    error::bind(module)?;
    core::bind(module)?;
    studio::bind(module)?;
    extern_struct_storage::bind(module)?;
    callback::bind();

    Ok(())
}
