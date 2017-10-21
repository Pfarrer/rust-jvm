use std::fs::File;

use classfile::util::read_u16;
use classfile::attributes;
use classfile::constants;

pub type Fields = Vec<Field>;

#[derive(Debug)]
pub struct Field {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: attributes::Attributes,
}

pub fn read(file: &mut File, constants: &constants::Constants) -> Fields {
    let fields_count = read_u16(file);
    let mut fields = Vec::with_capacity(fields_count as usize);

    for _ in 1..fields_count {
        fields.push(read_field(file, constants));
    }

    fields
}

fn read_field(file: &mut File, constants: &constants::Constants) -> Field {
    let access_flags = read_u16(file);
    let name_index = read_u16(file);
    let descriptor_index = read_u16(file);
    let attributes = attributes::read(file, constants);

    Field {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    }
}