use anyhow::{bail, Context, Result};
use model::class::{ClassConstant, ClassConstants};

use crate::{method_signature::parse_method_signature, type_signature::parse_type_signature};

use super::raw::RawConstant;

pub fn map(raw_constants: Vec<RawConstant>) -> Result<ClassConstants> {
    let constants = raw_constants
        .iter()
        .enumerate()
        .map(|(i, raw_constant)| match raw_constant {
            &RawConstant::None() => Ok(ClassConstant::Unused()),
            &RawConstant::Class(name_index) => {
                let class_name = unwrap_string(&raw_constants, name_index)?;

                Ok(ClassConstant::Class(class_name))
            }
            &RawConstant::Fieldref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index)?;
                let (field_name, type_name) =
                    unwrap_name_and_type(&raw_constants, name_and_type_index)?;
                let type_signature = parse_type_signature(&type_name)?;

                Ok(ClassConstant::Fieldref(
                    class_name,
                    field_name,
                    type_signature,
                ))
            }
            &RawConstant::Methodref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index)?;
                let (method_name, type_name) =
                    unwrap_name_and_type(&raw_constants, name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;

                Ok(ClassConstant::Methodref(
                    class_name,
                    method_name,
                    method_signature,
                ))
            }
            &RawConstant::InterfaceMethodref(class_index, name_and_type_index) => {
                let class_name = unwrap_class(&raw_constants, class_index)?;
                let (method_name, type_name) =
                    unwrap_name_and_type(&raw_constants, name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;

                Ok(ClassConstant::InterfaceMethodref(
                    class_name,
                    method_name,
                    method_signature,
                ))
            }
            &RawConstant::String(value_index) => {
                let value = unwrap_string(&raw_constants, value_index)?;

                Ok(ClassConstant::String(value))
            }
            &RawConstant::Integer(value) => Ok(ClassConstant::Integer(value)),
            &RawConstant::Float(value) => Ok(ClassConstant::Float(value)),
            &RawConstant::Long(value) => Ok(ClassConstant::Long(value)),
            &RawConstant::Double(value) => Ok(ClassConstant::Double(value)),
            &RawConstant::NameAndType(_, _) => {
                let (name, type_name) = unwrap_name_and_type(&raw_constants, i as u16)?;

                if type_name.chars().nth(0) == Some('(') {
                    let method_signature = parse_method_signature(&type_name)?;
                    Ok(ClassConstant::MethodNameAndType(name, method_signature))
                } else {
                    let type_signature = parse_type_signature(&type_name)?;
                    Ok(ClassConstant::FieldNameAndType(name, type_signature))
                }
            }
            &RawConstant::Utf8(ref val) => Ok(ClassConstant::Utf8(val.to_string())),
            &RawConstant::MethodHandle(ref _reference_kind, ref _reference_index) => Ok(ClassConstant::NotImplemented),
            &RawConstant::MethodType(ref descriptor_index) => {
                let type_name = unwrap_string(&raw_constants, *descriptor_index)?;
                let method_signature = parse_method_signature(&type_name)?;
                
                Ok(ClassConstant::MethodType(method_signature))
            },
            &RawConstant::Dynamic(ref bootstrap_method_attr_index, ref name_and_type_index) => {
                let (name, type_name) = unwrap_name_and_type(&raw_constants, *name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;

                Ok(ClassConstant::Dynamic(*bootstrap_method_attr_index, name, method_signature))
            },
            &RawConstant::InvokeDynamic(ref bootstrap_method_attr_index, ref name_and_type_index) => {
                let (name, type_name) = unwrap_name_and_type(&raw_constants, *name_and_type_index)?;
                let method_signature = parse_method_signature(&type_name)?;

                Ok(ClassConstant::InvokeDynamic(*bootstrap_method_attr_index, name, method_signature))
            },
        })
        .collect::<Result<_>>()?;

    Ok(constants)
}

fn unwrap_class(raw_constants: &Vec<RawConstant>, class_index: u16) -> Result<String> {
    match raw_constants
        .get(class_index as usize)
        .context("Raw constant not found")?
    {
        &RawConstant::Class(name_index) => unwrap_string(raw_constants, name_index),
        it => bail!("Expected Class but found {:?}", it),
    }
}

fn unwrap_name_and_type(raw_constants: &Vec<RawConstant>, index: u16) -> Result<(String, String)> {
    match raw_constants
        .get(index as usize)
        .context("Raw constant not found")?
    {
        &RawConstant::NameAndType(name_index, type_index) => {
            let name = unwrap_string(raw_constants, name_index)?;
            let type_name = unwrap_string(raw_constants, type_index)?;

            Ok((name, type_name))
        }
        it => bail!("Expected NameAndType but found {:?}", it),
    }
}

fn unwrap_string(raw_constants: &Vec<RawConstant>, index: u16) -> Result<String> {
    match raw_constants
        .get(index as usize)
        .context("Raw constant not found")?
    {
        &RawConstant::Utf8(ref val) => Ok(val.to_string()),
        it => bail!("Expected Utf8 but found {:?}", it),
    }
}
