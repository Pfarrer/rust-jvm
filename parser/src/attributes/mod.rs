use std::io::Read;

use anyhow::{bail, Result};
use class_constant_impl::ClassConstantAccessor;
use model::prelude::*;

use crate::util;

mod code;
mod constant_value;
mod source_file;

pub fn parse<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<ClassAttributes> {
    let attributes_count = util::read_u16(reader)? as usize;
    let mut attributes = Vec::with_capacity(attributes_count);

    for _ in 0..attributes_count {
        attributes.push(parse_attribute(reader, constants)?);
    }

    Ok(attributes)
}

pub fn parse_attribute<T: Read>(
    reader: &mut T,
    constants: &ClassConstants,
) -> Result<ClassAttribute> {
    let attribute_name_index = util::read_u16(reader)? as usize;
    let atrribute_name = constants.get_utf8_or(attribute_name_index)?;

    match atrribute_name.as_str() {
        "Code" => Ok(ClassAttribute::Code(code::parse(reader, constants)?)),
        "SourceFile" => Ok(ClassAttribute::SourceFile(source_file::parse(
            reader, constants,
        )?)),
        "ConstantValue" => Ok(ClassAttribute::ConstantValue(constant_value::parse(
            reader, constants,
        )?)),
        "LineNumberTable"
        | "LocalVariableTable"
        | "RuntimeVisibleAnnotations"
        | "InnerClasses"
        | "EnclosingMethod"
        | "Signature"
        | "StackMapTable"
        | "Exceptions"
        | "Deprecated"
        | "BootstrapMethods"
        | "NestHost"
        | "NestMembers"
        | "Synthetic"
        | "LocalVariableTypeTable"
         => {
            let attribute_length = util::read_u32(reader)? as usize;
            if attribute_length > 0 {
                util::read_raw(reader, attribute_length)?;
            }

            Ok(ClassAttribute::NotImplemented)
        }

        _ => bail!("Attribute not implemented: {atrribute_name}"),
    }
}
