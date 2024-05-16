use magnus::prelude::*;

type Result<T> = std::result::Result<T, magnus::Error>;

mod core;
mod studio;

mod macros;
pub use macros::*;

// FIXME functions returning string include null terminator

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<()> {
    let module = ruby.define_module("FMOD")?;

    module.const_set("VERSION", fmod::VERSION)?;

    core::bind(module)?;
    studio::bind(module)?;

    Ok(())
}
