use anyhow::{anyhow, bail, Result};
use classfile_parser::constant_info::ConstantInfo;
use classfile_parser::types::ClassFile;
use model::class::{ClassConstant, ClassConstants};

use crate::method_signature::parse_method_signature;
use crate::type_signature::parse_type_signature;

pub fn map(classfile: &ClassFile) -> Result<ClassConstants> {
    let constants = classfile
        .const_pool
        .iter()
        .map(|entry| match entry {
            ConstantInfo::Unusable => Ok(ClassConstant::Unused()),
            ConstantInfo::Utf8(constant) => Ok(ClassConstant::Utf8(constant.utf8_string.clone())),
            ConstantInfo::Integer(constant) => Ok(ClassConstant::Integer(constant.value)),
            ConstantInfo::Float(constant) => Ok(ClassConstant::Float(constant.value)),
            ConstantInfo::Long(constant) => Ok(ClassConstant::Long(constant.value)),
            ConstantInfo::Double(constant) => Ok(ClassConstant::Double(constant.value)),
            ConstantInfo::Class(constant) => Ok(ClassConstant::Class(unwrap_string(
                classfile,
                constant.name_index,
            )?)),
            ConstantInfo::String(constant) => Ok(ClassConstant::String(unwrap_string(
                classfile,
                constant.string_index,
            )?)),
            ConstantInfo::FieldRef(constant) => {
                let class_name = unwrap_class(classfile, constant.class_index)?;
                let (field_name, type_name) =
                    unwrap_name_and_type(classfile, constant.name_and_type_index)?;
                let type_signature = parse_type_signature(&type_name)?;
                Ok(ClassConstant::Fieldref(
                    class_name,
                    field_name,
                    type_signature,
                ))
            }
            ConstantInfo::MethodRef(constant) => {
                let class_name = unwrap_class(classfile, constant.class_index)?;
                let (field_name, type_name) =
                    unwrap_name_and_type(classfile, constant.name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;
                Ok(ClassConstant::Methodref(
                    class_name,
                    field_name,
                    method_signature,
                ))
            }
            ConstantInfo::InterfaceMethodRef(constant) => {
                let class_name = unwrap_class(classfile, constant.class_index)?;
                let (field_name, type_name) =
                    unwrap_name_and_type(classfile, constant.name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;
                Ok(ClassConstant::InterfaceMethodref(
                    class_name,
                    field_name,
                    method_signature,
                ))
            }
            ConstantInfo::NameAndType(constant) => {
                let name = unwrap_string(classfile, constant.name_index)?;
                let type_name = unwrap_string(classfile, constant.descriptor_index)?;
                if type_name.chars().nth(0) == Some('(') {
                    let method_signature = parse_method_signature(&type_name)?;
                    Ok(ClassConstant::MethodNameAndType(name, method_signature))
                } else {
                    let type_signature = parse_type_signature(&type_name)?;
                    Ok(ClassConstant::FieldNameAndType(name, type_signature))
                }
            }
            ConstantInfo::MethodHandle(constant) => Ok(ClassConstant::MethodHandle(
                constant.reference_kind,
                constant.reference_index,
            )),
            ConstantInfo::MethodType(constant) => {
                let type_name = unwrap_string(classfile, constant.descriptor_index)?;
                let method_signature = parse_method_signature(&type_name)?;
                Ok(ClassConstant::MethodType(method_signature))
            }
            ConstantInfo::InvokeDynamic(constant) => {
                let (method_name, type_name) =
                    unwrap_name_and_type(classfile, constant.name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;
                Ok(ClassConstant::InvokeDynamic(
                    constant.bootstrap_method_attr_index,
                    method_name,
                    method_signature,
                ))
            }
        })
        .collect::<Result<_>>()?;

    Ok(ClassConstants(constants))
}

fn get_entry(classfile: &ClassFile, index: u16) -> Result<&ConstantInfo> {
    classfile.const_pool.get(index as usize - 1).ok_or(anyhow!(
        "const_pool entry {} not available, const_pool length is {}",
        index,
        classfile.const_pool.len()
    ))
}

fn unwrap_class(classfile: &ClassFile, index: u16) -> Result<String> {
    match get_entry(classfile, index)? {
        &ConstantInfo::Class(ref constant) => unwrap_string(classfile, constant.name_index),
        it => bail!("Expected Class but found {:?} at index {}", it, index),
    }
}

fn unwrap_name_and_type(classfile: &ClassFile, index: u16) -> Result<(String, String)> {
    match get_entry(classfile, index)? {
        &ConstantInfo::NameAndType(ref constant) => {
            let name = unwrap_string(classfile, constant.name_index)?;
            let type_name = unwrap_string(classfile, constant.descriptor_index)?;

            Ok((name, type_name))
        }
        it => bail!("Expected NameAndType but found {:?} at index {}", it, index),
    }
}

fn unwrap_string(classfile: &ClassFile, index: u16) -> Result<String> {
    match get_entry(classfile, index)? {
        &ConstantInfo::Utf8(ref constant) => Ok(constant.utf8_string.clone()),
        it => bail!("Expected Utf8 but found {:?} at index {}", it, index),
    }
}
