use std::io::Read;

use anyhow::Result;
use class_constant_impl::ClassConstantAccessor;
use model::prelude::*;

use crate::util;

pub fn parse<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<String> {
    /*let attribute_length = */
    util::read_u32(reader)?;

    let class_name_index = util::read_u16(reader)? as usize;
    let class_name = constants.get_utf8_or(class_name_index)?.clone();

    Ok(class_name)
}
