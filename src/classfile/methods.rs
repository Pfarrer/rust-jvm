use std::fs::File;

use classfile::util::read_u16;
use classfile::attributes;
use classfile::constant_pool;

pub type Methods = Vec<Method>;

#[derive(Debug)]
pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: attributes::Attributes,
}

pub fn read(file: &mut File, constants: &constant_pool::Constants) -> Methods {
    let fields_count = read_u16(file);
    let mut methods = Vec::with_capacity(fields_count as usize);

    for _ in 1..fields_count {
        methods.push(read_method(file, constants));
    }

    methods
}

fn read_method(file: &mut File, constants: &constant_pool::Constants) -> Method {
    let access_flags = read_u16(file);
    let name_index = read_u16(file);
    let descriptor_index = read_u16(file);
    let attributes = attributes::read(file, constants);

    Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes
    }
}