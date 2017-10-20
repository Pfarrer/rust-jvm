use std::fs::File;
use std::io::Read;

use classfile::util::read_u16;
use classfile::util::conv::make_u32;
use classfile::constant_pool;

pub type Attributes = Vec<Attribute>;

#[derive(Debug)]
pub struct Attribute {
}

pub fn read(file: &mut File, constants: &constant_pool::Constants) -> Attributes {
    let attributes_count = read_u16(file);
    let mut attributes = Vec::with_capacity(attributes_count as usize);

    println!("attributes count: {}", attributes_count);

    for _ in 0..attributes_count {
        attributes.push(read_attribute(file, constants));
    }

    attributes
}

pub fn read_attribute(file: &mut File, constants: &constant_pool::Constants) -> Attribute {
    let attribute_name_index = read_u16(file);

    let mut attribute_length_bin = [0u8; 4];
    file.read(&mut attribute_length_bin).unwrap();
    let attribute_length = make_u32(attribute_length_bin);

    println!("name: {:#?}", constants[attribute_name_index as usize]);
    println!("attribute_length: {:b}", attribute_name_index);
    panic!();

}