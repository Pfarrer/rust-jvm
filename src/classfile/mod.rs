mod util;
mod attributes;
mod version;
mod constants;
mod class_info;
mod fields;
mod methods;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct Classfile {
    pub version: version::Version,
//    pub constants: constants::Constants,
//    pub class_info: class_info::ClassInfo,
//    pub fields: fields::Fields,
//    pub methods: methods::Methods,
//    pub attributes: attributes::Attributes,
}


pub fn load_file(filename: String) -> Classfile {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let version = version::read(&mut reader);
//    let constants = constants::read(&mut file);
//    let class_info = class_info::read(&mut file);
//    let fields = fields::read(&mut file, &constants);
//    let methods = methods::read(&mut file, &constants);
//    let attributes = attributes::read(&mut file, &constants);

    Classfile {
        version,
//        constants,
//        class_info,
//        fields,
//        methods,
//        attributes,
    }
}

