use std::io::Read;

use classfile::util;
use classfile::attributes;
use classfile::constants;

pub type Fields = Vec<Field>;

#[derive(Clone, Debug)]
pub struct Field {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: attributes::Attributes,
}

pub fn read(reader: &mut Read, constants: &constants::Constants) -> Fields {
    let fields_count = util::read_u16(reader);
    let mut fields = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
        fields.push(read_field(reader, constants));
    }

    fields
}

fn read_field(reader: &mut Read, constants: &constants::Constants) -> Field {
    let access_flags = util::read_u16(reader);
    let name_index = util::read_u16(reader);
    let descriptor_index = util::read_u16(reader);
    let attributes = attributes::read(reader, constants);

    Field {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    }
}