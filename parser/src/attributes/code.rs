use std::io::Read;

use crate::util;
use model::class::{ClassConstant, CodeAttribute, ExceptionTable};

pub fn read(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> CodeAttribute {
    /*let attribute_length = */
    util::read_u32(reader);

    let max_stack = util::read_u16(reader);
    let max_locals = util::read_u16(reader);

    let code_length = util::read_u32(reader);
    let code = util::read_raw(reader, code_length as usize);

    let exception_table = read_exception_table(reader);
    let attributes = crate::attributes::read(reader, constants);

    CodeAttribute {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    }
}

fn read_exception_table(reader: &mut impl Read) -> Vec<ExceptionTable> {
    let exception_table_length = util::read_u16(reader);
    let mut entries = Vec::with_capacity(exception_table_length as usize);

    for _ in 0..exception_table_length {
        entries.push(ExceptionTable {
            start_pc: util::read_u16(reader),
            end_pc: util::read_u16(reader),
            handler_pc: util::read_u16(reader),
            catch_type: util::read_u16(reader),
        })
    }

    entries
}
