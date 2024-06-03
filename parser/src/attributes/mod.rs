use std::io::Read;

use anyhow::{bail, Context, Result};
use model::prelude::*;

use crate::util;

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
    let atrribute_name = constants
        .get(attribute_name_index)
        .context(format!("get constant with index {}", attribute_name_index))?
        .expect_utf8()?;

    match atrribute_name.as_str() {
        // "Code" => Attribute::Code(code::read(reader, constants)),
        // "LineNumberTable" => Attribute::LineNumberTable(line_number_table::read(reader)),
        // "SourceFile" => Attribute::SourceFile(source_file::read(reader)),
        // "Exceptions" => Attribute::Exceptions(exceptions::read(reader)),
        // "Signature" => Attribute::Signature(signature::read(reader)),
        // "ConstantValue" => Attribute::ConstantValue(constant_value::read(reader)),

        // "RuntimeVisibleAnnotations" => {
        //     let attribute_length = util::read_u32(reader);
        //     util::read_raw(reader, attribute_length as usize);
        //     Attribute::NotImplemented
        // }

        // "InnerClasses" => {
        //     let attribute_length = util::read_u32(reader);
        //     util::read_raw(reader, attribute_length as usize);
        //     Attribute::NotImplemented
        // }

        // "EnclosingMethod" => {
        //     let attribute_length = util::read_u32(reader);
        //     assert_eq!(4, attribute_length);
        //     util::read_u32(reader);

        //     Attribute::NotImplemented
        // }

        // "Deprecated" => {
        //     /*let attribute_length = */
        //     util::read_u32(reader);
        //     Attribute::Deprecated
        // }

        _ => bail!("Attribute not implemented: {atrribute_name}"),
    }
}
