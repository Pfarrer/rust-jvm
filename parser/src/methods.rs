use anyhow::{bail, Result};
use classfile_parser::{method_info::MethodAccessFlags, types::ClassFile};
use enumset::EnumSet;
use model::class::{ClassConstants, ClassMethod, ClassMethods, MethodAccessFlag};

use crate::method_signature::parse_method_signature;

pub fn map(classfile: &ClassFile, constants: &ClassConstants) -> Result<ClassMethods> {
    classfile
        .methods
        .iter()
        .map(|method| {
            Ok(ClassMethod {
                access_flags: Wrap(method.access_flags).try_into()?,
                name: constants.get_string(method.name_index)?.into(),
                descriptor: parse_method_signature(constants.get_string(method.descriptor_index)?)?,
                attributes: vec![],
            })
        })
        .collect()
}

struct Wrap<T>(T);

impl TryFrom<Wrap<MethodAccessFlags>> for EnumSet<MethodAccessFlag> {
    type Error = anyhow::Error;

    fn try_from(value: Wrap<MethodAccessFlags>) -> std::result::Result<Self, Self::Error> {
        fn map_flag(flag: MethodAccessFlags) -> Result<MethodAccessFlag> {
            match flag {
                MethodAccessFlags::PUBLIC => Ok(MethodAccessFlag::Public),
                MethodAccessFlags::PRIVATE => Ok(MethodAccessFlag::Private),
                MethodAccessFlags::PROTECTED => Ok(MethodAccessFlag::Protected),
                MethodAccessFlags::STATIC => Ok(MethodAccessFlag::Static),
                MethodAccessFlags::FINAL => Ok(MethodAccessFlag::Final),
                MethodAccessFlags::SYNCHRONIZED => Ok(MethodAccessFlag::Synchronized),
                MethodAccessFlags::BRIDGE => Ok(MethodAccessFlag::Bridge),
                MethodAccessFlags::VARARGS => Ok(MethodAccessFlag::Varargs),
                MethodAccessFlags::NATIVE => Ok(MethodAccessFlag::Native),
                MethodAccessFlags::ABSTRACT => Ok(MethodAccessFlag::Abstract),
                MethodAccessFlags::STRICT => Ok(MethodAccessFlag::Strict),
                MethodAccessFlags::SYNTHETIC => Ok(MethodAccessFlag::Synthetic),
                it => bail!("Unexpected MethodAccessFlag: {:?}", it),
            }
        }

        value
            .0
            .iter()
            .try_fold(EnumSet::new(), |acc, flag| Ok(map_flag(flag)? | acc))
    }
}
