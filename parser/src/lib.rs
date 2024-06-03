use std::io::Read;

use anyhow::Result;
use model::class::JvmClass;

pub mod method_signature;
pub mod type_signature;

mod util;
mod version;
mod constants;
mod class_info;
mod fields;
mod methods;
mod attributes;

pub struct ClassfileParser {}

impl model::api::Parser for ClassfileParser {
    fn parse<T: Read>(&self, mut reader: &mut T) -> Result<JvmClass> {
        let version = version::parse(&mut reader)?;
        let constants = constants::parse(&mut reader)?;

        let access_flags = class_info::parse_access_flags(&mut reader)?;
        let this_class = class_info::parse_this_class(&mut reader, &constants)?;
        let super_class: Option<String> = class_info::parse_super_class(&mut reader, &constants)?;
        let interfaces = class_info::parse_interfaces(&mut reader, &constants)?;

        let fields = fields::parse(&mut reader, &constants)?;
        let methods = methods::parse(&mut reader, &constants)?;
        let attributes = attributes::parse(&mut reader, &constants)?;

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
