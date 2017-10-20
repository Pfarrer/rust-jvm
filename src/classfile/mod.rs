mod conv;
mod version;
mod constant_pool;

use std::fs::File;

#[derive(Debug)]
pub struct Classfile {
    pub version: version::Version,
    pub constant_pool: constant_pool::ConstantPool
}


pub fn load_file(filename: String) -> Classfile {
    let mut file = File::open(filename).unwrap();

    let version = version::read(&mut file);
    let constant_pool = constant_pool::read(&mut file);

    Classfile {
        version,
        constant_pool
    }
}

