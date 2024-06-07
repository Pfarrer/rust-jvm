use std::io::Read;

use crate::util;
use anyhow::{bail, Result};

#[derive(Clone, Debug)]
pub(crate) enum RawConstant {
    // This will be the first element of the constants pool for each class reader. This enables
    // easier handling of index parameters since Java class indexes are not 0 based.
    None(),

    // name_index
    Class(u16),

    // class_index, name_and_type_index
    Fieldref(u16, u16),

    // class_index, name_and_type_index
    Methodref(u16, u16),

    // class_index, name_and_type_index
    InterfaceMethodref(u16, u16),

    // string_index
    String(u16),

    // Value
    Integer(i32),

    // Value
    Float(f32),

    // Value
    Long(i64),

    // Value
    Double(f64),

    // name_index, descriptor_index
    NameAndType(u16, u16),

    // Value
    Utf8(String),

    // reference_kind, reference_index
    MethodHandle(u8, u16),

    // descriptor_index
    MethodType(u16),

    // bootstrap_method_attr_index, name_and_type_index
    Dynamic(u16, u16),

    // bootstrap_method_attr_index, name_and_type_index
    InvokeDynamic(u16, u16),
}

pub fn parse<T: Read>(reader: &mut T) -> Result<Vec<RawConstant>> {
    let constant_pool_count = util::read_u16(reader)?;
    let mut constants = Vec::with_capacity(constant_pool_count as usize);
    constants.push(RawConstant::None());

    while constants.len() < constant_pool_count as usize {
        let tag = util::read_u8(reader)?;

        constants.push(match tag {
            1 => parse_utf8(reader)?,
            3 => parse_integer(reader)?,
            4 => parse_float(reader)?,
            5 => parse_long(reader)?,
            6 => parse_double(reader)?,
            7 => parse_class(reader)?,
            8 => parse_string(reader)?,
            9 => parse_fieldref(reader)?,
            10 => parse_methodref(reader)?,
            11 => parse_interface_methodref(reader)?,
            12 => parse_name_and_type(reader)?,
            15 => parse_method_handle(reader)?,
            16 => parse_method_type(reader)?,
            17 => parse_dynamic(reader)?,
            18 => parse_invoke_dynamic(reader)?,
            _ => bail!("Unexpected Constant Pool Tag: {}", tag),
        });

        // In case of long and double, the next element of the constant pool must be empty
        if tag == 5 || tag == 6 {
            constants.push(RawConstant::None());
        }
    }

    Ok(constants)
}

fn parse_class<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let name_index = util::read_u16(reader)?;

    Ok(RawConstant::Class(name_index))
}

fn parse_fieldref<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let class_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::Fieldref(class_index, name_and_type_index))
}

fn parse_methodref<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let class_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::Methodref(class_index, name_and_type_index))
}

fn parse_interface_methodref<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let class_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::InterfaceMethodref(
        class_index,
        name_and_type_index,
    ))
}

fn parse_string<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let string_index = util::read_u16(reader)?;

    Ok(RawConstant::String(string_index))
}

fn parse_integer<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val = util::conv::to_i32(bin);

    Ok(RawConstant::Integer(val))
}

fn parse_long<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 8];
    reader.read(&mut bin).unwrap();
    let val = util::conv::to_i64(bin);

    Ok(RawConstant::Long(val))
}

fn parse_float<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val = util::conv::to_f32(bin);

    Ok(RawConstant::Float(val))
}

fn parse_double<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 8];
    reader.read(&mut bin).unwrap();
    let val = util::conv::to_f64(bin);

    Ok(RawConstant::Double(val))
}

fn parse_utf8<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let length = util::read_u16(reader)?;
    let bytes = util::read_raw(reader, length as usize)?;
    let str = cesu8::from_java_cesu8(&bytes)
        .unwrap_or_else(|_| String::from_utf8_lossy(&bytes))
        .into_owned();

    Ok(RawConstant::Utf8(str))
}

fn parse_name_and_type<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let name_index = util::read_u16(reader)?;
    let descriptor_index = util::read_u16(reader)?;

    Ok(RawConstant::NameAndType(name_index, descriptor_index))
}

fn parse_method_handle<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let reference_kind = util::read_u8(reader)?;
    let reference_index = util::read_u16(reader)?;

    Ok(RawConstant::MethodHandle(reference_kind, reference_index))
}

fn parse_method_type<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let descriptor_index = util::read_u16(reader)?;

    Ok(RawConstant::MethodType(descriptor_index))
}

fn parse_dynamic<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let bootstrap_method_attr_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::Dynamic(bootstrap_method_attr_index, name_and_type_index))
}

fn parse_invoke_dynamic<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let bootstrap_method_attr_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index))
}
