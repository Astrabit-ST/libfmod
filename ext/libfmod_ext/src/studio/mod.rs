use crate::Result;
use magnus::prelude::*;

pub fn bind(module: magnus::RModule) -> Result<()> {
    let module = module.define_module("Studio")?;

    Ok(())
}
