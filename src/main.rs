mod class_info;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        load_file(arg1);
    }
    else {
        panic!("Expect class file as first parameter.");
    }
}

fn load_file(filename: String) {
    let mut file = File::open(filename).unwrap();
    read_major_minor_version(&mut file);
    read_constant_pool(&mut file);

    let info = class_info::read(&mut file);
    println!("Class Info: {:?}", info);
}

fn read_constant_pool(file: &mut File) {
    let mut constant_pool_count_bin = [0u8; 2];
    file.read(&mut constant_pool_count_bin).unwrap();

    let constant_pool_count = make_u16(constant_pool_count_bin);
    println!("Constant Pool Count: {}", constant_pool_count);

    for x in 1..constant_pool_count {
        let mut tag_bin = [0u8; 1];
        file.read(&mut tag_bin).unwrap();
        let tag: u8 = tag_bin[0];

        print!("{} ", x);

        match tag {
            1 => {
                read_constant_pool_utf8(file)
            }
            3 => {
                read_constant_pool_integer(file)
            }
            4 => {
                read_constant_pool_float(file)
            }
            7 => {
                read_constant_pool_class(file)
            }
            8 => {
                let mut string_index_bin = [0u8; 2];
                file.read(&mut string_index_bin).unwrap();
                let string_index: u16 = make_u16(string_index_bin);
                println!("- Constant String Index: {} ", string_index);
            }
            9 => {
                read_constant_pool_fieldref(file)
            }
            10 => {
                read_constant_pool_methodref(file)
            }
            11 => {
                read_constant_pool_interface_methodref(file)
            }
            12 => {
                read_constant_pool_name_and_type(file)
            }
            _ => println!("- Unimplemented Tag: {}", tag)
        }
    }
}

fn read_constant_pool_class(file: &mut File) {
    let mut name_index_bin = [0u8; 2];
    file.read(&mut name_index_bin).unwrap();
    let name_index: u16 = make_u16(name_index_bin);

    println!("- Constant Class Index: {} ", name_index);
}

fn read_constant_pool_fieldref(file: &mut File) {
    let mut class_index_bin = [0u8; 2];
    file.read(&mut class_index_bin).unwrap();
    let class_index: u16 = make_u16(class_index_bin);

    let mut name_and_type_index_bin = [0u8; 2];
    file.read(&mut name_and_type_index_bin).unwrap();
    let name_and_type_index: u16 = make_u16(name_and_type_index_bin);

    println!("- Constant Fieldref: class_index: {}, name_and_type_index: {} ", class_index, name_and_type_index);
}

fn read_constant_pool_methodref(file: &mut File) {
    let mut class_index_bin = [0u8; 2];
    file.read(&mut class_index_bin).unwrap();
    let class_index: u16 = make_u16(class_index_bin);

    let mut name_and_type_index_bin = [0u8; 2];
    file.read(&mut name_and_type_index_bin).unwrap();
    let name_and_type_index: u16 = make_u16(name_and_type_index_bin);

    println!("- Constant Methodref: class_index: {}, name_and_type_index: {} ", class_index, name_and_type_index);
}

fn read_constant_pool_interface_methodref(file: &mut File) {
    let mut class_index_bin = [0u8; 2];
    file.read(&mut class_index_bin).unwrap();
    let class_index: u16 = make_u16(class_index_bin);

    let mut name_and_type_index_bin = [0u8; 2];
    file.read(&mut name_and_type_index_bin).unwrap();
    let name_and_type_index: u16 = make_u16(name_and_type_index_bin);

    println!("- Constant InterfaceMethodref: class_index: {}, name_and_type_index: {} ", class_index, name_and_type_index);
}

fn read_constant_pool_integer(file: &mut File) {
    let mut bin = [0u8; 4];
    file.read(&mut bin).unwrap();
    let val: i32 = make_i32(bin);

    println!("- Constant Integer Index: {} ", val);
}

fn read_constant_pool_float(file: &mut File) {
    let mut bin = [0u8; 4];
    file.read(&mut bin).unwrap();
    let val: f32 = make_f32(bin);

    println!("- Constant Float Index: {} ", val);
}

fn read_constant_pool_utf8(file: &mut File) {
    let mut length_bin = [0u8; 2];
    file.read(&mut length_bin).unwrap();
    let length= make_u16(length_bin);

    // println!("- Constant Utf8 Len: {} ", length);

    let mut byte = [0u8; 1];
    let mut bytes = Vec::new();
    for _ in 0..length {
        file.read(&mut byte).unwrap();
        bytes.push(byte[0]);
    }

    let val = String::from_utf8(bytes).expect("Found invalid UTF-8");
    println!("- Constant Utf8: {} ", val);
}

fn read_constant_pool_name_and_type(file: &mut File) {
    let mut name_index_bin = [0u8; 2];
    file.read(&mut name_index_bin).unwrap();
    let name_index: u16 = make_u16(name_index_bin);

    let mut descriptor_index_bin = [0u8; 2];
    file.read(&mut descriptor_index_bin).unwrap();
    let descriptor_index: u16 = make_u16(descriptor_index_bin);

    println!("- Constant NameAndType: Name Index: {}, Descriptor Index: {} ", name_index, descriptor_index);
}

fn read_major_minor_version(file: &mut File) {
    let mut magic = [0u8; 4];
    file.read(&mut magic).unwrap();

    if !validate_magic(magic) {
        panic!("No valid Java class file.");
    }

    let mut major_bin = [0u8; 2];
    file.read(&mut major_bin).unwrap();

    let mut minor_bin = [0u8; 2];
    file.read(&mut minor_bin).unwrap();

    println!("Java Version: {}.{}", make_u16(major_bin), make_u16(minor_bin));
}

fn validate_magic(magic: [u8; 4]) -> bool {
    let expected: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    return magic.eq(&expected);
}

fn make_u16(val: [u8; 2]) -> u16 {
    let high: u16 = (val[0] as u16) << 8;
    let low: u16 = val[1] as u16;

    return high + low;
}

fn make_f32(val: [u8; 4]) -> f32 {
    unsafe {
        std::mem::transmute::<[u8; 4], f32>(val)
    }
}

fn make_i32(val: [u8; 4]) -> i32 {
    unsafe {
        std::mem::transmute::<[u8; 4], i32>(val)
    }
}