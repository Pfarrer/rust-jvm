mod code;

use std::fs::File;
use std::io::Read;

use classfile::util::read_u16;
use classfile::util::conv::make_u32;
use classfile::constants;

pub type Attributes = Vec<Attribute>;

#[derive(Debug)]
pub enum Attribute {
    Code(code::CodeAttribute),
}

pub fn read(file: &mut File, constants: &constants::Constants) -> Attributes {
    let attributes_count = read_u16(file);
    let mut attributes = Vec::with_capacity(attributes_count as usize);

    for _ in 0..attributes_count {
        attributes.push(read_attribute(file, constants));
    }

    attributes
}

pub fn read_attribute(file: &mut File, constants: &constants::Constants) -> Attribute {
    let attribute_name_index = read_u16(file) as usize;

    match constants[attribute_name_index] {
        constants::Constant::Utf8(ref name) => match name.as_ref() {
            "Code" => Attribute::Code(code::read(file, constants)),

            name => panic!("Attribute name not implemented: {}", name),
        },
        _ => panic!("Invalid constant indexed by attribute")
    }
}