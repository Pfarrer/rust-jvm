use std::io::Read;

use anyhow::{Context, Result};
use enumset::EnumSet;
use model::prelude::*;

use crate::util;

pub fn parse_access_flags<T: Read>(reader: &mut T) -> Result<EnumSet<ClassAccessFlag>> {
    let access_flags = util::read_u16(reader)?;

    let mut enumset = EnumSet::new();

    if access_flags & 0x0001 > 0 {
        enumset.insert(ClassAccessFlag::Public);
    }
    if access_flags & 0x0010 > 0 {
        enumset.insert(ClassAccessFlag::Final);
    }
    if access_flags & 0x0020 > 0 {
        enumset.insert(ClassAccessFlag::Super);
    }
    if access_flags & 0x0200 > 0 {
        enumset.insert(ClassAccessFlag::Interface);
    }
    if access_flags & 0x0400 > 0 {
        enumset.insert(ClassAccessFlag::Abstract);
    }
    if access_flags & 0x1000 > 0 {
        enumset.insert(ClassAccessFlag::Synthetic);
    }
    if access_flags & 0x2000 > 0 {
        enumset.insert(ClassAccessFlag::Annotation);
    }
    if access_flags & 0x4000 > 0 {
        enumset.insert(ClassAccessFlag::Enum);
    }
    if access_flags & 0x8000 > 0 {
        enumset.insert(ClassAccessFlag::Module);
    }

    Ok(enumset)
}

pub fn parse_this_class<'a, T: Read>(reader: &mut T, constants: &'a ClassConstants) -> Result<&'a String> {
    let this_class_index = util::read_u16(reader)? as usize;
    let class_name = constants
        .get(this_class_index)
        .context(format!("get constant with index {}", this_class_index))?
        .expect_class()?;

    Ok(class_name)
}

pub fn parse_super_class<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<Option<String>> {
    let super_class_index = util::read_u16(reader)? as usize;
    
    let super_class = if super_class_index > 0 {
        let super_class_name = constants
        .get(super_class_index)
        .context(format!("get constant with index {}", super_class_index))?
        .expect_class()?;

        Some(super_class_name.clone())
    } else {
        None
    };

    Ok(super_class)
}

pub fn parse_interfaces<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<Vec<String>> {
    let interfaces_count = util::read_u16(reader)? as usize;
    let mut interfaces = Vec::with_capacity(interfaces_count);

    for _ in 0..interfaces_count {
        let interface_name_index = util::read_u16(reader)? as usize;
        let interface_name = constants
            .get(interface_name_index)
            .context(format!("get constant with index {}", interface_name_index))?
            .expect_class()?;

        interfaces.push(interface_name.clone());
    }

    Ok(interfaces)
}
