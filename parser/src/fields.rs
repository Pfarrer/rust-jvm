use anyhow::{bail, Result};
use classfile_parser::{field_info::FieldAccessFlags, types::ClassFile};
use enumset::EnumSet;
use model::class::{ClassConstants, ClassField, ClassFields, FieldAccessFlag};

use crate::type_signature::parse_type_signature;

pub fn map(classfile: &ClassFile, constants: &ClassConstants) -> Result<ClassFields> {
    classfile
        .fields
        .iter()
        .map(|field| {
            Ok(ClassField {
                access_flags: Wrap(field.access_flags).try_into()?,
                name: constants.get_string(field.name_index)?.into(),
                descriptor: parse_type_signature(constants.get_string(field.descriptor_index)?)?,
                attributes: vec![],
            })
        })
        .collect()
}

struct Wrap<T>(T);

impl TryFrom<Wrap<FieldAccessFlags>> for EnumSet<FieldAccessFlag> {
    type Error = anyhow::Error;

    fn try_from(value: Wrap<FieldAccessFlags>) -> std::result::Result<Self, Self::Error> {
        fn map_flag(flag: FieldAccessFlags) -> Result<FieldAccessFlag> {
            match flag {
                FieldAccessFlags::PUBLIC => Ok(FieldAccessFlag::Public),
                FieldAccessFlags::PRIVATE => Ok(FieldAccessFlag::Private),
                FieldAccessFlags::PROTECTED => Ok(FieldAccessFlag::Protected),
                FieldAccessFlags::STATIC => Ok(FieldAccessFlag::Static),
                FieldAccessFlags::FINAL => Ok(FieldAccessFlag::Final),
                FieldAccessFlags::VOLATILE => Ok(FieldAccessFlag::Volatile),
                FieldAccessFlags::TRANSIENT => Ok(FieldAccessFlag::Transient),
                FieldAccessFlags::SYNTHETIC => Ok(FieldAccessFlag::Synthetic),
                FieldAccessFlags::ANNOTATION => Ok(FieldAccessFlag::Annotation),
                FieldAccessFlags::ENUM => Ok(FieldAccessFlag::Enum),
                it => bail!("Unexpected FieldAccessFlag: {:?}", it),
            }
        }

        value
            .0
            .iter()
            .try_fold(EnumSet::new(), |acc, flag| Ok(map_flag(flag)? | acc))
    }
}
