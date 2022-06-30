pub mod accessor;
mod raw;

use crate::util::{parse_method_signature, parse_type_signature};
use model::class::ClassConstant;
use std::io::Read;

pub fn read(reader: &mut impl Read) -> Vec<ClassConstant> {
    let raw_constants = raw::read(reader);

    process_raw_constants(raw_constants)
}

fn process_raw_constants(raw_constants: Vec<raw::Constant>) -> Vec<ClassConstant> {
    raw_constants
        .iter()
        .enumerate()
        .map(|(_, raw_constant)| match raw_constant {
            &raw::Constant::None() => ClassConstant::None(),
            &raw::Constant::Class(name_index) => {
                let class_name = unwrap_string(&raw_constants, name_index);

                ClassConstant::Class(class_name)
            }
            &raw::Constant::Fieldref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index);
                let (field_name, type_name) =
                    unwrap_name_and_type(&raw_constants, name_and_type_index);
                let type_signature = parse_type_signature(&type_name);

                ClassConstant::Fieldref(class_name, field_name, type_signature)
            }
            &raw::Constant::Methodref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index);
                let (method_name, type_name) =
                    unwrap_name_and_type(&raw_constants, name_and_type_index);
                let method_signature = parse_method_signature(&type_name);

                ClassConstant::Methodref(class_name, method_name, method_signature)
            }
            &raw::Constant::InterfaceMethodref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index);
                let (method_name, type_name) =
                    unwrap_name_and_type(&raw_constants, name_and_type_index);
                let method_signature = parse_method_signature(&type_name);

                ClassConstant::InterfaceMethodref(class_name, method_name, method_signature)
            }
            &raw::Constant::String(value_index) => {
                let value = unwrap_string(&raw_constants, value_index);

                ClassConstant::String(value)
            }
            &raw::Constant::Integer(value) => ClassConstant::Integer(value),
            &raw::Constant::Float(value) => ClassConstant::Float(value),
            &raw::Constant::Long(value) => ClassConstant::Long(value),
            &raw::Constant::Double(value) => ClassConstant::Double(value),
            &raw::Constant::NameAndType(name_index, type_index) => {
                let name = unwrap_string(&raw_constants, name_index);
                let type_name = unwrap_string(&raw_constants, type_index);
                if type_name.chars().nth(0) == Some('(') {
                    let method_signature = parse_method_signature(&type_name);
                    ClassConstant::MethodNameAndType(name, method_signature)
                } else {
                    let type_signature = parse_type_signature(&type_name);
                    ClassConstant::FieldNameAndType(name, type_signature)
                }
            }
            &raw::Constant::MethodHandle(reference_kind, reference_index) => {
                ClassConstant::MethodHandle(reference_kind, reference_index)
            }
            &raw::Constant::MethodType(descriptor_index) => {
                let value = unwrap_string(&raw_constants, descriptor_index);

                ClassConstant::MethodType(value)
            }
            &raw::Constant::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index) => {
                ClassConstant::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index)
            }
            &raw::Constant::Utf8(ref val) => ClassConstant::Utf8(val.to_string()),
        })
        .collect()
}

fn unwrap_class(raw_constants: &Vec<raw::Constant>, class_index: u16) -> String {
    match raw_constants.get(class_index as usize).unwrap() {
        &raw::Constant::Class(name_index) => unwrap_string(raw_constants, name_index),
        it => panic!("Expected Class but found {:?}", it),
    }
}

fn unwrap_name_and_type(raw_constants: &Vec<raw::Constant>, index: u16) -> (String, String) {
    match raw_constants.get(index as usize).unwrap() {
        &raw::Constant::NameAndType(name_index, type_index) => {
            let name = unwrap_string(raw_constants, name_index);
            let type_name = unwrap_string(raw_constants, type_index);

            (name, type_name)
        }
        it => panic!("Expected NameAndType but found {:?}", it),
    }
}

fn unwrap_string(raw_constants: &Vec<raw::Constant>, index: u16) -> String {
    match raw_constants.get(index as usize).unwrap() {
        &raw::Constant::Utf8(ref val) => val.to_string(),
        it => panic!("Expected Utf8 but found {:?}", it),
    }
}
