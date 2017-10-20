mod util;
mod attributes;
mod version;
mod constant_pool;
mod class_info;
mod fields;
mod methods;

use std::fs::File;

#[derive(Debug)]
pub struct Classfile {
    pub version: version::Version,
    pub constants: constant_pool::Constants,
    pub class_info: class_info::ClassInfo,
    pub fields: fields::Fields,
    pub methods: methods::Methods,
//    pub attributes: attributes::Attributes,
}


pub fn load_file(filename: String) -> Classfile {
    let mut file = File::open(filename).unwrap();

    let version = version::read(&mut file);
    let constants = constant_pool::read(&mut file);
    let class_info = class_info::read(&mut file);
    let fields = fields::read(&mut file, &constants);
    let methods = methods::read(&mut file, &constants);
    //let attributes = attributes::read(&mut file, &constants);

    Classfile {
        version,
        constants,
        class_info,
        fields,
        methods,
//        attributes
    }
}

