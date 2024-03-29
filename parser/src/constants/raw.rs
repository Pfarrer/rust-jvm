use crate::util;
use std::io::Read;

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
    Long(i64),

    // Value
    Double(f64),

    // name_index, descriptor_index
    NameAndType(u16, u16),

    // Value
    Utf8(String),

    // reference_kind, reference_index
    MethodHandle(u8, u16),

    // descriptor_index
    MethodType(u16),

    // bootstrap_method_attr_index, name_and_type_index
    InvokeDynamic(u16, u16),
}

pub fn read(reader: &mut impl Read) -> Vec<Constant> {
    let constant_pool_count = util::read_u16(reader);
    let mut constants = Vec::with_capacity(constant_pool_count as usize);
    constants.push(Constant::None());

    let mut tag_bin = [0u8; 1];
    while constants.len() < constant_pool_count as usize {
        reader.read(&mut tag_bin).unwrap();

        constants.push(match tag_bin[0] {
            1 => read_utf8(reader),
            3 => read_integer(reader),
            4 => read_float(reader),
            5 => read_long(reader),
            6 => read_double(reader),
            7 => read_class(reader),
            8 => read_string(reader),
            9 => read_fieldref(reader),
            10 => read_methodref(reader),
            11 => read_interface_methodref(reader),
            12 => read_name_and_type(reader),
            15 => read_method_handle(reader),
            16 => read_method_type(reader),
            18 => read_invoke_dynamic(reader),
            _ => panic!("Unexpected Constant Pool Tag: {}", tag_bin[0]),
        });

        // In case of long and double, the next element of the constant pool must be empty
        if tag_bin[0] == 5 || tag_bin[0] == 6 {
            constants.push(Constant::None());
        }
    }

    constants
}

fn read_class(reader: &mut impl Read) -> Constant {
    let name_index = util::read_u16(reader);

    Constant::Class(name_index)
}

fn read_fieldref(reader: &mut impl Read) -> Constant {
    let class_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::Fieldref(class_index, name_and_type_index)
}

fn read_methodref(reader: &mut impl Read) -> Constant {
    let class_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::Methodref(class_index, name_and_type_index)
}

fn read_interface_methodref(reader: &mut impl Read) -> Constant {
    let class_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::InterfaceMethodref(class_index, name_and_type_index)
}

fn read_string(reader: &mut impl Read) -> Constant {
    let string_index = util::read_u16(reader);

    Constant::String(string_index)
}

fn read_integer(reader: &mut impl Read) -> Constant {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val: i32 = util::to_i32(bin);

    Constant::Integer(val)
}

fn read_long(reader: &mut impl Read) -> Constant {
    let mut bin = [0u8; 8];
    reader.read(&mut bin).unwrap();
    let val: i64 = util::to_i64(bin);

    Constant::Long(val)
}

fn read_float(reader: &mut impl Read) -> Constant {
    let mut bin = [0u8; 4];
    reader.read(&mut bin).unwrap();
    let val: f32 = util::to_f32(bin);

    Constant::Float(val)
}

fn read_double(reader: &mut impl Read) -> Constant {
    let mut bin = [0u8; 8];
    reader.read(&mut bin).unwrap();
    let val: f64 = util::to_f64(bin);

    Constant::Double(val)
}

fn read_utf8(reader: &mut impl Read) -> Constant {
    let length = util::read_u16(reader);
    let bytes = util::read_raw(reader, length as usize);
    let val = String::from_utf8(bytes).expect("Found invalid UTF-8");

    Constant::Utf8(val)
}

fn read_name_and_type(reader: &mut impl Read) -> Constant {
    let name_index = util::read_u16(reader);
    let descriptor_index = util::read_u16(reader);

    Constant::NameAndType(name_index, descriptor_index)
}

fn read_method_handle(reader: &mut impl Read) -> Constant {
    let reference_kind = util::read_raw(reader, 1)[0];
    let reference_index = util::read_u16(reader);

    Constant::MethodHandle(reference_kind, reference_index)
}

fn read_method_type(reader: &mut impl Read) -> Constant {
    let descriptor_index = util::read_u16(reader);

    Constant::MethodType(descriptor_index)
}

fn read_invoke_dynamic(reader: &mut impl Read) -> Constant {
    let bootstrap_method_attr_index = util::read_u16(reader);
    let name_and_type_index = util::read_u16(reader);

    Constant::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index)
}
