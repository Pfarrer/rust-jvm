mod code;
mod line_number_table;
mod source_file;

use std::io::Read;

use classfile::util;
use classfile::constants;

pub type Attributes = Vec<Attribute>;

#[derive(Debug)]
pub enum Attribute {
    Code(code::CodeAttribute),
    LineNumberTable(line_number_table::LineNumberTable),
    SourceFile(u16),
}

pub fn read(reader: &mut Read, constants: &constants::Constants) -> Attributes {
    let attributes_count = util::read_u16(reader);
    let mut attributes = Vec::with_capacity(attributes_count as usize);

    for _ in 0..attributes_count {
        attributes.push(read_attribute(reader, constants));
    }

    attributes
}

pub fn read_attribute(reader: &mut Read, constants: &constants::Constants) -> Attribute {
    let attribute_name_index = util::read_u16(reader) as usize;

    match constants[attribute_name_index] {
        constants::Constant::Utf8(ref name) => match name.as_ref() {
            "Code" => Attribute::Code(code::read(reader, constants)),
            "LineNumberTable" => Attribute::LineNumberTable(line_number_table::read(reader)),
            "SourceFile" => Attribute::SourceFile(source_file::read(reader)),

            name => panic!("Attribute not implemented: {}", name),
        },
        _ => panic!("Invalid constant indexed by attribute: {}\n{:?}", attribute_name_index, constants)
    }
}