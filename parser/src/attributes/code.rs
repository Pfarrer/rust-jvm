use std::io::Read;

use anyhow::Result;
use class_constant_impl::ClassConstantAccessor;
use model::prelude::*;

use crate::util;
use crate::attributes;

pub fn parse<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<Code> {
    /*let attribute_length = */util::read_u32(reader)?;

    let max_stack = util::read_u16(reader)?;
    let max_locals = util::read_u16(reader)?;

    let code_length = util::read_u32(reader)? as usize;
    let code = util::read_raw(reader, code_length)?;

    let exception_table = parse_exception_table(reader, constants)?;
    let attributes = attributes::parse(reader, constants)?;

    Ok(Code {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    })
}

fn parse_exception_table<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<Vec<ExceptionTable>> {
    let exception_table_length = util::read_u16(reader)? as usize;
    let mut entries = Vec::with_capacity(exception_table_length);

    for _ in 0..exception_table_length {
        entries.push(ExceptionTable {
            start_pc: util::read_u16(reader)?,
            end_pc: util::read_u16(reader)?,
            handler_pc: util::read_u16(reader)?,
            catch_type: parse_exception_table_catch_type(reader, constants)?,
        })
    }

    Ok(entries)
}

fn parse_exception_table_catch_type<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<Option<String>> {
    let class_name_index = util::read_u16(reader)? as usize;
    let class_name = if class_name_index == 0 {
        None
    } else {
        Some(constants.get_class_or(class_name_index)?.clone())
    };

    Ok(class_name)
}