use std::io::Read;

use classfile::util;
use classfile::constants;
use classfile::attributes;

#[derive(Debug)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTable>,
    attributes: attributes::Attributes,
}

#[derive(Debug)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub fn read(reader: &mut Read, constants: &constants::Constants) -> CodeAttribute {
    /*let attribute_length = */util::read_u32(reader);

    let max_stack = util::read_u16(reader);
    let max_locals = util::read_u16(reader);

    let code_length = util::read_u32(reader);
    let code = util::read_raw(reader, code_length as usize);

    let exception_table = read_exception_table(reader);
    let attributes = attributes::read(reader, constants);

    CodeAttribute {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    }
}

fn read_exception_table(reader: &mut Read) -> Vec<ExceptionTable> {
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