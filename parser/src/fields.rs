use std::io::Read;

use crate::util;
use crate::{attributes, constants};
use model::class::{ClassConstant, ClassField};

pub fn read(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> Vec<ClassField> {
    let fields_count = util::read_u16(reader);
    (0..fields_count)
        .map(|_| read_field(reader, constants))
        .collect()
}

fn read_field(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> ClassField {
    let access_flags = util::read_u16(reader);
    let name_index = util::read_u16(reader);
    let descriptor_index = util::read_u16(reader);
    let attributes = attributes::read(reader, constants);

    let name = constants::accessor::unwrap_string(constants, name_index);

    let descriptor_string = constants::accessor::unwrap_string(constants, descriptor_index);
    let descriptor = util::parse_type_signature(&descriptor_string);

    ClassField {
        access_flags,
        name,
        descriptor,
        attributes,
    }
}
