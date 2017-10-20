mod util;
mod version;
mod constant_pool;
mod class_info;

use std::fs::File;

#[derive(Debug)]
pub struct Classfile {
    pub version: version::Version,
    pub constant_pool: constant_pool::ConstantPool,
    pub class_info: class_info::ClassInfo,
}


pub fn load_file(filename: String) -> Classfile {
    let mut file = File::open(filename).unwrap();

    let version = version::read(&mut file);
    let constant_pool = constant_pool::read(&mut file);
    let class_info = class_info::read(&mut file);

    Classfile {
        version,
        constant_pool,
        class_info,
    }
}

