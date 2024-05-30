use anyhow::{bail, Result};
use classfile_parser::{types::ClassFile, ClassAccessFlags};
use enumset::EnumSet;
use model::class::{ClassConstants, ClassAccessFlag};

pub fn map(classfile: &ClassFile, constants: &ClassConstants) -> Result<(EnumSet<ClassAccessFlag>, String, Option<String>, Vec<String>)> {
    let access_flags = Wrap(classfile.access_flags).try_into()?;
    let this_class = constants.get_class(classfile.this_class)?.into();

    let super_class = if classfile.super_class > 0 {
        Some(constants.get_class(classfile.super_class)?.to_owned())
    } else {
        None
    };

    let interfaces: Vec<_> = classfile.interfaces.iter().map(|index|
        constants.get_class(*index).map(|s| s.to_owned())
    ).collect::<Result<_>>()?;

    Ok((access_flags, this_class, super_class, interfaces))
}

struct Wrap<T>(T);

impl TryFrom<Wrap<ClassAccessFlags>> for EnumSet<ClassAccessFlag> {
    type Error = anyhow::Error;

    fn try_from(value: Wrap<ClassAccessFlags>) -> std::result::Result<Self, Self::Error> {
        fn map_flag(flag: ClassAccessFlags) -> Result<ClassAccessFlag> {
            match flag {
                ClassAccessFlags::PUBLIC => Ok(ClassAccessFlag::Public),
                ClassAccessFlags::FINAL => Ok(ClassAccessFlag::Final),
                ClassAccessFlags::SUPER => Ok(ClassAccessFlag::Super),
                ClassAccessFlags::INTERFACE => Ok(ClassAccessFlag::Interface),
                ClassAccessFlags::ABSTRACT => Ok(ClassAccessFlag::Abstract),
                ClassAccessFlags::SYNTHETIC => Ok(ClassAccessFlag::Synthetic),
                ClassAccessFlags::ANNOTATION => Ok(ClassAccessFlag::Annotation),
                ClassAccessFlags::ENUM => Ok(ClassAccessFlag::Enum),
                it => bail!("Unexpected ClassAccessFlag: {:?}", it),
            }
        }

        value
            .0
            .iter()
            .try_fold(EnumSet::new(), |acc, flag| Ok(map_flag(flag)? | acc))
    }
}
