mod util;
mod version;
pub mod constants;
mod class_info;
mod fields;
mod methods;
pub mod attributes;

use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Debug)]
pub struct Classfile {
    pub version: version::Version,
    pub constants: constants::Constants,
    pub class_info: class_info::ClassInfo,
    pub fields: fields::Fields,
    pub methods: methods::Methods,
    pub attributes: attributes::Attributes,
}

//pub const ACC_PUBLIC: u16 = 0x0001; // Declared public; may be accessed from outside its package.
//pub const ACC_PRIVATE: u16 = 0x0002; // Declared private; usable only within the defining class.
//pub const ACC_PROTECTED: u16 = 0x0004; // Declared protected; may be accessed within subclasses.
//pub const ACC_STATIC: u16 = 0x0008; // Declared static.
//    final = 0x0010, // Declared final; no subclasses allowed.
//    super = 0x0020, // Treat superclass methods specially when invoked by the invokespecial instruction.
//    interface = 0x0200, // Is an interface, not a class.
pub const ACC_NATIVE: u16 = 0x0100; // Declared native; implemented in a language other than Java.
pub const ACC_ABSTRACT: u16 = 0x0400; // Declared abstract; must not be instantiated.
//    ACC_SYNTHETIC = 0x1000, // Declared synthetic; not present in the source code.
//    ACC_ANNOTATION = 0x2000, //	Declared as an annotation type.
//    ACC_ENUM = 0x4000, // Declared as an enum type.

pub type Constant = constants::Constant;
pub type Method = methods::Method;

pub fn load_file(filename: &String) -> Classfile {
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

