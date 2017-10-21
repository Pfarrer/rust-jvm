use std::fs::File;
use std::io::Read;

use classfile::util::read_u16;
use classfile::util::read_u32;
use classfile::util::read_bytes;
use classfile::constants;
use classfile::attributes;

#[derive(Debug)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    attributes: attributes::Attributes,
}

#[derive(Debug)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub fn read(file: &mut File, constants: &constants::Constants) -> CodeAttribute {
    /*let attribute_length = */read_u32(file);

    let max_stack = read_u16(file);
    let max_locals = read_u16(file);

    let code_length = read_u32(file);
    let code = read_bytes(file, code_length as usize);

    let exception_table = read_exception_table(file);
    let attributes = attributes::read(file, constants);

    CodeAttribute {
        max_stack,
        max_locals,
        code,
        attributes,
    }
}

fn read_exception_table(file: &mut File) -> Vec<ExceptionTable> {
    let exception_table_length = read_u32(file);
    let mut entries = Vec::with_capacity(exception_table_length as usize);

    for _ in 0..exception_table_length {
        entries.push(ExceptionTable {
            start_pc: read_u16(file),
            end_pc: read_u16(file),
            handler_pc: read_u16(file),
            catch_type: read_u16(file),
        })
    }

    entries
}