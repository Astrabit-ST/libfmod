use crate::Result;

pub mod structs;

pub fn bind(module: magnus::RModule) -> Result<()> {
    structs::bind(module)?;

    Ok(())
}
