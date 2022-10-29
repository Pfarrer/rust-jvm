use std::io::Read;

use crate::util;
use crate::{attributes, constants};
use model::class::{ClassConstant, ClassMethod};

pub fn read(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> Vec<ClassMethod> {
    let fields_count = util::read_u16(reader);
    let mut methods = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
        methods.push(read_method(reader, constants));
    }

    methods
}

fn read_method(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> ClassMethod {
    let access_flags = util::read_u16(reader);
    let name_index = util::read_u16(reader);
    let descriptor_index = util::read_u16(reader);
    let attributes = attributes::read(reader, constants);

    let name = constants::accessor::unwrap_string(constants, name_index);

    let descriptor_string = constants::accessor::unwrap_string(constants, descriptor_index);
    let descriptor = util::parse_method_signature(&descriptor_string);

    ClassMethod {
        access_flags,
        name,
        descriptor,
        attributes,
    }
}
