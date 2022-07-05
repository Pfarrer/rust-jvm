mod bootstrap_methods;
mod code;
mod constant_value;
mod exceptions;
mod signature;
mod source_file;
mod source_line_number;

use std::io::Read;

use crate::util;
use model::class::{ClassAttribute, ClassConstant};

pub fn read(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> Vec<ClassAttribute> {
    let attributes_count = util::read_u16(reader);
    (0..attributes_count)
        .map(|_| read_attribute(reader, constants))
        .collect()
}

pub fn read_attribute(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> ClassAttribute {
    let attribute_name_index = util::read_u16(reader) as usize;

    match constants[attribute_name_index] {
        ClassConstant::Utf8(ref name) => match name.as_ref() {
            "Code" => ClassAttribute::Code(code::read(reader, constants)),
            "LineNumberTable" => ClassAttribute::LineNumberTable(source_line_number::read(reader)),
            "SourceFile" => ClassAttribute::SourceFile(source_file::read(reader)),
            "Exceptions" => ClassAttribute::Exceptions(exceptions::read(reader)),
            "Signature" => ClassAttribute::Signature(signature::read(reader)),
            "ConstantValue" => ClassAttribute::ConstantValue(constant_value::read(reader)),
            "BootstrapMethods" => ClassAttribute::BootstrapMethods(bootstrap_methods::read(reader)),

            "RuntimeVisibleAnnotations" => {
                let attribute_length = util::read_u32(reader);
                util::read_raw(reader, attribute_length as usize);
                ClassAttribute::NotImplemented
            }

            "InnerClasses" => {
                let attribute_length = util::read_u32(reader);
                util::read_raw(reader, attribute_length as usize);
                ClassAttribute::NotImplemented
            }

            "EnclosingMethod" => {
                let attribute_length = util::read_u32(reader);
                assert_eq!(4, attribute_length);
                util::read_u32(reader);

                ClassAttribute::NotImplemented
            }

            "Deprecated" => {
                let attribute_length = util::read_u32(reader);
                assert_eq!(0, attribute_length);
                ClassAttribute::Deprecated
            }

            "LocalVariableTable" => {
                let attribute_length = util::read_u32(reader);
                util::read_raw(reader, attribute_length as usize);
                ClassAttribute::NotImplemented
            }

            "LocalVariableTypeTable" => {
                let attribute_length = util::read_u32(reader);
                util::read_raw(reader, attribute_length as usize);
                ClassAttribute::NotImplemented
            }

            "StackMapTable" => {
                let attribute_length = util::read_u32(reader);
                util::read_raw(reader, attribute_length as usize);
                ClassAttribute::NotImplemented
            }

            name => panic!("Attribute not implemented: {}", name),
        },
        _ => panic!(
            "Invalid constant indexed by attribute: {}",
            attribute_name_index
        ),
    }
}
