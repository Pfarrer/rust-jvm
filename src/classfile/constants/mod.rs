mod raw;

use std::io::Read;

pub type Constants = Vec<Constant>;

#[derive(Clone, Debug)]
pub enum Constant {
    // This will be the first element of the constants pool for each class reader. This enables
    // easier handling of index parameters since Java class indexes are not 0 based.
    None(),

    // name_index
    Class(String),

    // class_name, field_name, type_descriptor
    Fieldref(String, String, String),

    // class_name, method_name, method_signature
    Methodref(String, String, String),

    // class_name, method_name, method_signature
    InterfaceMethodref(String, String, String),

    // class_index, name_and_type_index
//    InterfaceMethodref(u16, u16),

    // string_index
    String(String),

    // Value
    Integer(i32),

    // Value
    Float(f32),

    // Value
    Long(i64),

    // Value
    Double(f64),

    // name, descriptor
    NameAndType(String, String),

    // Value
    Utf8(String),

    // reference_kind, reference_index
//    MethodHandle(u8, u16),

    // descriptor_index
//    MethodType(u16),

    // bootstrap_method_attr_index, name_and_type_index
//    InvokeDynamic(u16, u16),
}

pub fn read(reader: &mut Read) -> Constants {
    let raw_constants = raw::read(reader);

    process_raw_constants(raw_constants)
}

fn process_raw_constants(raw_constants: raw::Constants) -> Constants {
    raw_constants.iter().enumerate().map(|(i, raw_constant)| {
        match raw_constant {
            &raw::Constant::None() => Constant::None(),
            &raw::Constant::Class(name_index) => {
                let class_name = unwrap_string(&raw_constants, name_index);

                Constant::Class(class_name)
            },
            &raw::Constant::Fieldref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index);
                let (field_name, type_name) = unwrap_name_and_type(&raw_constants, name_and_type_index);

                Constant::Fieldref(class_name, field_name, type_name)
            },
            &raw::Constant::Methodref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index);
                let (method_name, type_name) = unwrap_name_and_type(&raw_constants, name_and_type_index);

                Constant::Methodref(class_name, method_name, type_name)
            },
            &raw::Constant::InterfaceMethodref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index);
                let (method_name, type_name) = unwrap_name_and_type(&raw_constants, name_and_type_index);

                Constant::InterfaceMethodref(class_name, method_name, type_name)
            },
            &raw::Constant::String(value_index) => {
                let value = unwrap_string(&raw_constants, value_index);

                Constant::String(value)
            },
            &raw::Constant::Integer(value) => Constant::Integer(value),
            &raw::Constant::Float(value) => Constant::Float(value),
            &raw::Constant::Long(value) => Constant::Long(value),
            &raw::Constant::Double(value) => Constant::Double(value),
            &raw::Constant::NameAndType(_, _) => {
                let (name, type_val) = unwrap_name_and_type(&raw_constants, i as u16);

                Constant::NameAndType(name, type_val)
            },
            &raw::Constant::Utf8(ref val) => Constant::Utf8(val.to_string()),
            it => panic!("Mapping from raw constant to processed constant not implemented for {:?}", it),
        }
    }).collect()
}

fn unwrap_class(raw_constants: &raw::Constants, class_index: u16) -> String {
    match raw_constants.get(class_index as usize).unwrap() {
        &raw::Constant::Class(name_index) => unwrap_string(raw_constants, name_index),
        it => panic!("Expected Class but found {:?}", it),
    }
}

fn unwrap_name_and_type(raw_constants: &raw::Constants, index: u16) -> (String, String) {
    match raw_constants.get(index as usize).unwrap() {
        &raw::Constant::NameAndType(name_index, type_index) => {
            let name = unwrap_string(raw_constants, name_index);
            let type_name = unwrap_string(raw_constants, type_index);

            (name, type_name)
        },
        it => panic!("Expected NameAndType but found {:?}", it),
    }
}

fn unwrap_string(raw_constants: &raw::Constants, index: u16) -> String {
    match raw_constants.get(index as usize).unwrap() {
        &raw::Constant::Utf8(ref val) => val.to_string(),
        it => panic!("Expected Utf8 but found {:?}", it),
    }
}
