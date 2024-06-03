use std::io::Read;

use anyhow::Result;
use enumset::EnumSet;
use model::{class::{ClassAccessFlag, ClassConstant, ClassConstants}, constant_value};

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

pub fn parse_this_class<T: Read>(reader: &mut T, constants: &ClassConstants) -> Result<String> {
    let this_class_index = util::read_u16(reader)?;
    let class_name = constant_value!(constants.get(this_class_index), ClassConstant::Class)?;

    Ok(class_name)
}

// pub fn parse<T: Read>(reader: &mut T) -> Result<ClassInfo> {
//     pub this_class: String,
//     pub super_class: Option<String>,
//     pub interfaces: Vec<String>,

//     let access_flags = util::read_u16(reader);
//     let this_class = util::read_u16(reader);
//     let super_class = util::read_u16(reader);

//     let interfaces_count = util::read_u16(reader);
//     let mut interfaces = Vec::with_capacity(interfaces_count as usize);
//     for _ in 0..interfaces_count {
//         let interface = util::read_u16(reader);
//         interfaces.push(interface);
//     }

//     ClassInfo {
//         access_flags,
//         this_class,
//         super_class,
//         interfaces,
//     }
// }

// pub fn parse<T: Read>(reader: &mut T) -> Result<ClassInfo> {
//     pub this_class: String,
//     pub super_class: Option<String>,
//     pub interfaces: Vec<String>,

//     let access_flags = util::read_u16(reader);
//     let this_class = util::read_u16(reader);
//     let super_class = util::read_u16(reader);

//     let interfaces_count = util::read_u16(reader);
//     let mut interfaces = Vec::with_capacity(interfaces_count as usize);
//     for _ in 0..interfaces_count {
//         let interface = util::read_u16(reader);
//         interfaces.push(interface);
//     }

//     ClassInfo {
//         access_flags,
//         this_class,
//         super_class,
//         interfaces,
//     }
// }