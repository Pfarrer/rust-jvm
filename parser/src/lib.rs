use anyhow::{bail, Result};
use model::class::{ClassVersion, JvmClass};
use std::io::Read;

pub mod method_signature;
pub mod type_signature;

mod attributes;
mod class_info;
mod constants;
mod fields;
mod methods;

pub struct ClassfileParser {}

impl model::api::Parser for ClassfileParser {
    fn parse<T: Read>(&self, reader: &mut T) -> Result<JvmClass> {
        let classfile =
            classfile_parser::parse_class_from_reader(reader, "unknown_path".to_string())
                .or_else(|err| bail!(err))?;

        let version = ClassVersion {
            major: classfile.major_version,
            minor: classfile.minor_version,
        };
        let constants = constants::map(&classfile)?;

        let (access_flags, this_class, super_class, interfaces) =
            class_info::map(&classfile, &constants)?;
        let fields = fields::map(&classfile, &constants)?;
        let methods = methods::map(&classfile, &constants)?;
        let attributes = attributes::map(&classfile, &constants)?;

        Ok(JvmClass {
            version,
            constants,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}
