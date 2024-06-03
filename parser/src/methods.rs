use std::io::Read;

use anyhow::{Context, Result};
use class_constant_impl::ConstantAccessor;
use enumset::EnumSet;
use model::prelude::*;

use crate::{attributes, method_signature::parse_method_signature, util};

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
    let name = constants.expect_utf8_at(name_index)?.clone();
    
    let descriptor_index = util::read_u16(reader)? as usize;
    let descriptor_string = constants
        .get(descriptor_index)
        .context(format!("get constant with index {}", descriptor_index))?
        .expect_utf8()?;
    let descriptor = parse_method_signature(descriptor_string)?;
    
    let attributes = attributes::parse(reader, constants)?;

    Ok(ClassMethod {
        access_flags,
        name,
        descriptor,
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