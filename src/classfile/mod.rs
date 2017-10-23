mod util;
mod version;
mod constants;
mod class_info;
mod fields;
mod methods;
mod attributes;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct Classfile {
    pub version: version::Version,
    pub constants: constants::Constants,
    pub class_info: class_info::ClassInfo,
    pub fields: fields::Fields,
    pub methods: methods::Methods,
    pub attributes: attributes::Attributes,
}

pub fn load_file(filename: String) -> Classfile {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let version = version::read(&mut reader);
    let constants = constants::read(&mut reader);
    let class_info = class_info::read(&mut reader);
    let fields = fields::read(&mut reader, &constants);
    let methods = methods::read(&mut reader, &constants);
    let attributes = attributes::read(&mut reader, &constants);

    Classfile {
        version,
        constants,
        class_info,
        fields,
        methods,
        attributes,
    }
}

