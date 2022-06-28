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
        let class_info = class_info::read(reader);
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

#[cfg(test)]
mod tests {
    use super::*;
    use model::api::Parser;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn parse_file_empty_class() {
        let pathbuf = self::testdata_path().join("Empty.class");
        let file = File::open(pathbuf).unwrap();
        let mut reader = BufReader::new(file);
        let class = ClassfileParser {}.parse(&mut reader);
        assert_eq!(61, class.version.major);
        assert_eq!(0, class.version.minor);
        assert_eq!(0, class.fields.len());
        assert_eq!(2, class.methods.len());
    }

    fn testdata_path() -> std::path::PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&manifest_dir).join("testdata").to_owned()
    }
}
