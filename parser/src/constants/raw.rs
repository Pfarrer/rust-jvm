use std::io::Read;

use anyhow::{anyhow, bail, Result};
use crate::util;

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
//    MethodHandle(u8, u16),

    // descriptor_index
//    MethodType(u16),

    // bootstrap_method_attr_index, name_and_type_index
//    InvokeDynamic(u16, u16),
}

pub fn parse<T: Read>(reader: &mut T) -> Result<Vec<RawConstant>> {
    let constant_pool_count = util::read_u16(reader)?;
    let mut constants = Vec::with_capacity(constant_pool_count as usize);
    constants.push(RawConstant::None());

    let mut tag_bin = [0u8; 1];
    while constants.len() < constant_pool_count as usize {
        reader.read(&mut tag_bin).unwrap();

        constants.push(match tag_bin[0] {
            1 => read_utf8(reader)?,
            3 => read_integer(reader)?,
            4 => read_float(reader)?,
            5 => read_long(reader)?,
            6 => read_double(reader)?,
            7 => read_class(reader)?,
            8 => read_string(reader)?,
            9 => read_fieldref(reader)?,
            10 => read_methodref(reader)?,
            11 => read_interface_methodref(reader)?,
            12 => read_name_and_type(reader)?,
            _ => bail!("Unexpected Constant Pool Tag: {}", tag_bin[0])
        });

        // In case of long and double, the next element of the constant pool must be empty
        if tag_bin[0] == 5 || tag_bin[0] == 6 {
            constants.push(RawConstant::None());
        }
    }

    Ok(constants)
}

fn read_class<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let name_index = util::read_u16(reader)?;

    Ok(RawConstant::Class(name_index))
}

fn read_fieldref<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let class_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::Fieldref(class_index, name_and_type_index))
}

fn read_methodref<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let class_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::Methodref(class_index, name_and_type_index))
}

fn read_interface_methodref<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let class_index = util::read_u16(reader)?;
    let name_and_type_index = util::read_u16(reader)?;

    Ok(RawConstant::InterfaceMethodref(class_index, name_and_type_index))
}

fn read_string<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let string_index = util::read_u16(reader)?;

    Ok(RawConstant::String(string_index))
}

fn read_integer<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val: i32 = util::conv::to_i32(bin);

    Ok(RawConstant::Integer(val))
}

fn read_long<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 8];
    reader.read(&mut bin).unwrap();
    let val: i64 = util::conv::to_i64(bin);

    Ok(RawConstant::Long(val))
}

fn read_float<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val: f32 = util::conv::to_f32(bin);

    Ok(RawConstant::Float(val))
}

fn read_double<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let mut bin = [0u8; 8];
    reader.read(&mut bin).unwrap();
    let val: f64 = util::conv::to_f64(bin);

    Ok(RawConstant::Double(val))
}

fn read_utf8<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let length = util::read_u16(reader)?;
    let bytes = util::read_raw(reader, length as usize)?;
    let val = String::from_utf8(bytes).map_err(|err| anyhow!(err))?;

    Ok(RawConstant::Utf8(val))
}

fn read_name_and_type<T: Read>(reader: &mut T) -> Result<RawConstant> {
    let name_index = util::read_u16(reader)?;
    let descriptor_index = util::read_u16(reader)?;

    Ok(RawConstant::NameAndType(name_index, descriptor_index))
}