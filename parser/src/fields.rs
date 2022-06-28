use std::io::Read;

use crate::{attributes, constants};
use crate::util;
use model::class::{ClassConstant, ClassField};

pub fn read(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> Vec<ClassField> {
    let fields_count = util::read_u16(reader);
    let mut fields = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
        fields.push(read_field(reader, constants));
    }

    fields
}

fn read_field(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> ClassField {
    let access_flags = util::read_u16(reader);
    let name_index = util::read_u16(reader);
    let name = constants::accessor::unwrap_string(constants, name_index);
    let descriptor_index = util::read_u16(reader);
    let attributes = attributes::read(reader, constants);

    ClassField {
        access_flags,
        name,
        descriptor_index,
        attributes,
    }
}
