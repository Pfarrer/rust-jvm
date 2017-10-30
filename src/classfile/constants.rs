use std::io::Read;

use classfile::util;

pub type Constants = Vec<Constant>;

#[derive(Clone, Debug)]
pub enum Constant {
    // This will be the first element of the constants pool for each class reader. This enables
    // easier handling of index parameters since Java class indexes are not 0 based.
    None(),

    // name_index
    Class(u16),

    // class_index, name_and_type_index
    Fieldref(u16, u16),

    // class_index, name_and_type_index
    Methodref(u16, u16),

    // class_index, name_and_type_index
    InterfaceMethodref(u16, u16),

    // string_index
    String(u16),

    // Value
    Integer(i32),

    // Value
    Float(f32),

    // Value
//    Long(i64),

    // Value
//    Double(f64),

    // name_index, descriptor_index
    NameAndType(u16, u16),

    // Value
    Utf8(String),

    // reference_kind, reference_index
//    MethodHandle(u8, u16),

    // descriptor_index
//    MethodType(u16),

    // bootstrap_method_attr_index, name_and_type_index
//    InvokeDynamic(u16, u16),
}

pub fn read(reader: &mut Read) -> Constants {
    let constant_pool_count = util::read_u16(reader);
    let mut constants = Vec::with_capacity(constant_pool_count as usize);
    constants.push(Constant::None());

    let mut tag_bin = [0u8; 1];
    for _ in 0..constant_pool_count-1 {
        reader.read(&mut tag_bin).unwrap();

        constants.push(match tag_bin[0] {
            1 => read_utf8(reader),
            3 => read_integer(reader),
            4 => read_float(reader),
            7 => read_class(reader),
            8 => read_string(reader),
            9 => read_fieldref(reader),
            10 => read_methodref(reader),
            11 => read_interface_methodref(reader),
            12 => read_name_and_type(reader),
            _ => panic!("Unexpected Constant Pool Tag: {}", tag_bin[0])
        })
    }

    constants
}

fn read_class(reader: &mut Read) -> Constant {
    let name_index = util::read_u16(reader);

    Constant::Class(name_index)
}

fn read_fieldref(reader: &mut Read) -> Constant {
    let class_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::Fieldref(class_index, name_and_type_index)
}

fn read_methodref(reader: &mut Read) -> Constant {
    let class_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::Methodref(class_index, name_and_type_index)
}

fn read_interface_methodref(reader: &mut Read) -> Constant {
    let class_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::InterfaceMethodref(class_index, name_and_type_index)
}

fn read_string(reader: &mut Read) -> Constant {
    let string_index = util::read_u16(reader);

    Constant::String(string_index)
}

fn read_integer(reader: &mut Read) -> Constant {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val: i32 = util::conv::to_i32(bin);

    Constant::Integer(val)
}

fn read_float(reader: &mut Read) -> Constant {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val: f32 = util::conv::to_f32(bin);

    Constant::Float(val)
}

fn read_utf8(reader: &mut Read) -> Constant {
    let length = util::read_u16(reader);
    let bytes = util::read_raw(reader, length as usize);
    let val = String::from_utf8(bytes).expect("Found invalid UTF-8");

    Constant::Utf8(val)
}

fn read_name_and_type(reader: &mut Read) -> Constant {
    let name_index = util::read_u16(reader);
    let descriptor_index = util::read_u16(reader);

    Constant::NameAndType(name_index, descriptor_index)
}