use std::io::Read;

use anyhow::Result;
use enumset::EnumSet;
use model::prelude::*;

use crate::util;

pub fn parse<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<ClassMethods> {
    let fields_count = util::read_u16(reader)?;
    let mut methods = Vec::with_capacity(fields_count as usize);

    for _ in 0..fields_count {
        methods.push(parse_method(reader, constants)?);
    }

    Ok(methods)
}

fn parse_method<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<ClassMethod> {
    let access_flags = parse_access_flags(reader)?;
    let name_index = util::read_u16(reader)? as usize;
    let descriptor_index = util::read_u16(reader)? as usize;
    let attributes = attributes::parse(reader, constants)?;

    Ok(ClassMethod {
        access_flags,
        name_index,
        descriptor_index,
        attributes
    })
}

fn parse_access_flags<T: Read>(reader: &mut T) -> Result<EnumSet<MethodAccessFlag>> {
    let access_flags = util::read_u16(reader)?;

    let mut enumset = EnumSet::new();

    if access_flags & 0x0001 > 0 {
        enumset.insert(MethodAccessFlag::Public);
    }
    if access_flags & 0x0002 > 0 {
        enumset.insert(MethodAccessFlag::Private);
    }
    if access_flags & 0x0004 > 0 {
        enumset.insert(MethodAccessFlag::Protected);
    }
    if access_flags & 0x0008 > 0 {
        enumset.insert(MethodAccessFlag::Static);
    }
    if access_flags & 0x0010 > 0 {
        enumset.insert(MethodAccessFlag::Final);
    }
    if access_flags & 0x0020 > 0 {
        enumset.insert(MethodAccessFlag::Synchronized);
    }
    if access_flags & 0x0040 > 0 {
        enumset.insert(MethodAccessFlag::Bridge);
    }
    if access_flags & 0x0080 > 0 {
        enumset.insert(MethodAccessFlag::Varargs);
    }
    if access_flags & 0x0100 > 0 {
        enumset.insert(MethodAccessFlag::Native);
    }
    if access_flags & 0x0400 > 0 {
        enumset.insert(MethodAccessFlag::Abstract);
    }
    if access_flags & 0x0800 > 0 {
        enumset.insert(MethodAccessFlag::Strict);
    }
    if access_flags & 0x1000 > 0 {
        enumset.insert(MethodAccessFlag::Synthetic);
    }

    Ok(enumset)
}