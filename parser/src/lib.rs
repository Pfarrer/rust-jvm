use model::class::JvmClass;
use std::io::Read;

mod attributes;
mod class_info;
mod constants;
mod fields;
mod methods;
mod util;
mod version;

pub use util::{parse_method_signature, parse_type_signature};

pub struct ClassfileParser {}

impl model::api::Parser for ClassfileParser {
    fn parse<T: Read>(&self, reader: &mut T) -> JvmClass {
        let version = version::read(reader);
        let constants = constants::read(reader);
        let class_info = class_info::read(reader, &constants);
        let fields = fields::read(reader, &constants);
        let methods = methods::read(reader, &constants);
        let attributes = attributes::read(reader, &constants);

        let jvm_class = JvmClass {
            version,
            constants,
            class_info,
            fields,
            methods,
            attributes,
        };

        jvm_class
    }
}