use std::io::Read;

use crate::util;
use model::class::{ClassConstant, ClassInfo};
use crate::constants::accessor::unwrap_class;

pub fn read(reader: &mut impl Read, constants: &Vec<ClassConstant>) -> ClassInfo {
    let access_flags = util::read_u16(reader);
    let this_class_index = util::read_u16(reader);
    let super_class_index = util::read_u16(reader);

    let interfaces_count = util::read_u16(reader);
    let interfaces = (0..interfaces_count).map(|_| util::read_u16(reader)).collect();

    let this_class = unwrap_class(constants, this_class_index).unwrap();
    let super_class = unwrap_class(constants, super_class_index);

    ClassInfo {
        access_flags,
        this_class,
        super_class,
        interfaces,
    }
}
