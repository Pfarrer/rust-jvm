use std::io::Read;

use classfile::util;
use classfile::attributes;
use classfile::constants;

pub type Methods = Vec<Method>;

#[derive(Clone, Debug)]
pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: attributes::Attributes,
}

pub fn read(reader: &mut Read, constants: &constants::Constants) -> Methods {
    let fields_count = util::read_u16(reader);
    let mut methods = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
        methods.push(read_method(reader, constants));
    }

    methods
}

fn read_method(reader: &mut Read, constants: &constants::Constants) -> Method {
    let access_flags = util::read_u16(reader);
    let name_index = util::read_u16(reader);
    let descriptor_index = util::read_u16(reader);
    let attributes = attributes::read(reader, constants);

    Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes
    }
}