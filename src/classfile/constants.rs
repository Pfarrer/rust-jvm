use std::fs::File;
use std::io::Read;

use classfile::util::read_u16;
use classfile::util::read_bytes;
use classfile::util::conv::make_i32;
use classfile::util::conv::make_f32;

pub type Constants = Vec<Constant>;

#[derive(Debug)]
pub enum Constant {
    // This will be the first element of the constants pool for each class file. This enables
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

pub fn read(file: &mut File) -> Constants {
    let constants_count = read_u16(file);
    let mut items = Vec::with_capacity(constants_count as usize + 1);
    items.push(Constant::None());

    let mut tag_bin = [0u8; 1];
    for _ in 1..constants_count {
        file.read(&mut tag_bin).unwrap();

        items.push(match tag_bin[0] {
            1 => read_utf8(file),
            3 => read_integer(file),
            4 => read_float(file),
            7 => read_class(file),
            8 => read_string(file),
            9 => read_fieldref(file),
            10 => read_methodref(file),
            11 => read_interface_methodref(file),
            12 => read_name_and_type(file),
            _ => panic!("Unexpected Constant Pool Tag: {}", tag_bin[0])
        })
    }

    items
}

fn read_class(file: &mut File) -> Constant {
    let name_index = read_u16(file);

    Constant::Class(name_index)
}

fn read_fieldref(file: &mut File) -> Constant {
    let class_index = read_u16(file);
    let name_and_type_index = read_u16(file);

    Constant::Fieldref(class_index, name_and_type_index)
}

fn read_methodref(file: &mut File) -> Constant {
    let class_index = read_u16(file);
    let name_and_type_index = read_u16(file);

    Constant::Methodref(class_index, name_and_type_index)
}

fn read_interface_methodref(file: &mut File) -> Constant {
    let class_index = read_u16(file);
    let name_and_type_index = read_u16(file);

    Constant::InterfaceMethodref(class_index, name_and_type_index)
}

fn read_string(file: &mut File) -> Constant {
    let string_index = read_u16(file);

    Constant::String(string_index)
}

fn read_integer(file: &mut File) -> Constant {
    let mut bin = [0u8; 4];
    file.read(&mut bin).unwrap();
    let val: i32 = make_i32(bin);

    Constant::Integer(val)
}

fn read_float(file: &mut File) -> Constant {
    let mut bin = [0u8; 4];
    file.read(&mut bin).unwrap();
    let val: f32 = make_f32(bin);

    Constant::Float(val)
}

fn read_utf8(file: &mut File) -> Constant {
    let length = read_u16(file);
    let bytes = read_bytes(file, length as usize);
    let val = String::from_utf8(bytes).expect("Found invalid UTF-8");

    Constant::Utf8(val)
}

fn read_name_and_type(file: &mut File) -> Constant {
    let name_index = read_u16(file);
    let descriptor_index = read_u16(file);

    Constant::NameAndType(name_index, descriptor_index)
}