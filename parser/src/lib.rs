use std::io::Read;

use anyhow::Result;
use model::class::JvmClass;

pub mod method_signature;
pub mod type_signature;

mod util;
mod version;
mod constants;
mod class_info;

pub struct ClassfileParser {}

impl model::api::Parser for ClassfileParser {
    fn parse<T: Read>(&self, reader: &mut T) -> Result<JvmClass> {
        let version = version::parse(&mut reader)?;
        let constants = constants::parse(&mut reader)?;

        let access_flags = class_info::parse_access_flags(&mut reader)?;
        let this_class = class_info::parse_this_class(&mut reader)?;
        
        
        // let fields = fields::read(&mut reader, &constants)?;
        // let methods = methods::read(&mut reader, &constants)?;
        // let attributes = attributes::read(&mut reader, &constants)?;

        Ok(JvmClass {
            version,
            constants,

            access_flags,
            this_class,

            // fields,
            // methods,
            // attributes,
        })
    }
}
