use std::io::Read;

use anyhow::Result;
use model::class::ClassConstants;

mod raw;
mod mapper;

pub fn parse<T: Read>(reader: &mut T) -> Result<ClassConstants> {
    let raw_constants = raw::parse(reader)?;
    let constants = mapper::map(raw_constants)?;
    Ok(constants)
}
