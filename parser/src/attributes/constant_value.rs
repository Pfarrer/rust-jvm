use std::io::Read;

use anyhow::Context;
use anyhow::Result;
use model::prelude::*;

use crate::util;

pub fn parse<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<ClassConstant> {
    /*let attribute_length = */util::read_u32(reader)?;

    let constant_index = util::read_u16(reader)? as usize;
    let constant = constants.get(constant_index).context(format!("get constant with index {constant_index}"))?.clone();

    Ok(constant)
}
