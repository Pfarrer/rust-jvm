use std::io::Read;

use anyhow::Result;
use enumset::EnumSet;
use model::prelude::*;

use crate::util;

pub fn parse<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<ClassFields> {
    let fields_count = util::read_u16(reader)? as usize;
    let mut fields = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
        fields.push(parse_field(reader, constants)?);
    }

    Ok(fields)
}

fn parse_field<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<ClassField> {
    let access_flags = parse_access_flags(reader)?;
    let name_index = util::read_u16(reader)? as usize;
    let descriptor_index = util::read_u16(reader)?  as usize;
    let attributes = attributes::parse(reader, constants);

    Ok(ClassField {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}


fn parse_access_flags<T: Read>(reader: &mut T) -> Result<EnumSet<FieldAccessFlag>> {
    let access_flags = util::read_u16(reader)?;

    let mut enumset = EnumSet::new();

    if access_flags & 0x0001 > 0 {
        enumset.insert(FieldAccessFlag::Public);
    }
    if access_flags & 0x0002 > 0 {
        enumset.insert(FieldAccessFlag::Private);
    }
    if access_flags & 0x0004 > 0 {
        enumset.insert(FieldAccessFlag::Protected);
    }
    if access_flags & 0x0008 > 0 {
        enumset.insert(FieldAccessFlag::Static);
    }
    if access_flags & 0x0010 > 0 {
        enumset.insert(FieldAccessFlag::Final);
    }
    if access_flags & 0x0040 > 0 {
        enumset.insert(FieldAccessFlag::Volatile);
    }
    if access_flags & 0x0080 > 0 {
        enumset.insert(FieldAccessFlag::Transient);
    }
    if access_flags & 0x1000 > 0 {
        enumset.insert(FieldAccessFlag::Synthetic);
    }
    if access_flags & 0x4000 > 0 {
        enumset.insert(FieldAccessFlag::Enum);
    }

    Ok(enumset)
}