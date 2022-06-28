use model::class::{ClassAttribute, ClassConstant, ClassMethod, CodeAttribute, JvmClass, TypeSignature};
use crate::array::Array;
use crate::{ClassHierarchy, Primitive, Vm};
use crate::instance::Instance;

pub fn get_utf8_value(classfile: &JvmClass, index: usize) -> String {
    match classfile.constants.get(index).unwrap() {
        &ClassConstant::Utf8(ref val) => val.clone(),
        it => panic!("Expected Utf8 but found: {:?}", it),
    }
}

pub fn get_class_path(classfile: &JvmClass) -> String {
    match classfile.constants.get(classfile.class_info.this_class as usize).unwrap() {
        &ClassConstant::Class(ref path) => path.clone(),
        it => panic!("Expected Class but found: {:?}", it),
    }
}

pub fn find_method(vm: &Vm, root_class_path: &String, name: &String, signature: &String) -> (JvmClass, ClassMethod) {
    // Loop through class hierarchy
    let hierarchy_iter = ClassHierarchy::hierarchy_iter(vm, root_class_path);
    for (class, _, _) in hierarchy_iter {
        match find_method_in_classfile(&class, name, signature) {
            Some(method) => return (class, method),
            None => (),
        };
    }

    panic!("ClassMethod not found: {}.{}{}", root_class_path, name, signature);
}

pub fn find_method_in_classfile(jvm_class: &JvmClass, name: &String, signature: &String) -> Option<ClassMethod> {
    jvm_class.methods.iter().find(|&method| {
        let correct_name = match jvm_class.constants.get(method.name_index).unwrap() {
            &ClassConstant::Utf8(ref val) => name.eq(val),
            _ => panic!("Invalid class file"),
        };

        if !correct_name {
            return false;
        }

        let correct_signature = match jvm_class.constants.get(method.descriptor_index).unwrap() {
            &ClassConstant::Utf8(ref val) => signature.eq(val),
            _ => panic!("Invalid class file"),
        };

        if !correct_signature {
            return false;
        }

        // Make sure, method is not abstract
        method.access_flags & JvmClass::ACC_ABSTRACT == 0
    }).map(|m| m.clone())
}

pub fn find_code(method: &ClassMethod) -> Option<&CodeAttribute> {
    for attr in method.attributes.iter() {
        if let &ClassAttribute::Code(ref code) = attr {
            return Some(code);
        }
    }

    None
}

pub fn read_u16_code(code: &Vec<u8>, pc: u16) -> u16 {
    let indexbyte1: u16 = (*code.get((pc + 1) as usize).unwrap() as u16) << 8;
    let indexbyte2 = (*code.get((pc + 2) as usize).unwrap()) as u16;

    indexbyte1 + indexbyte2
}

pub fn read_i16_code(code: &Vec<u8>, pc: u16) -> i16 {
    let indexbyte1: u16 = (*code.get((pc + 1) as usize).unwrap() as u16) << 8;
    let indexbyte2 = (*code.get((pc + 2) as usize).unwrap()) as u16;

    (indexbyte1 | indexbyte2) as i16
}

pub fn read_i32_code(code: &Vec<u8>, pc: u16, offset: u16) -> i32 {
    let byte1 = *code.get((pc + offset) as usize).unwrap() as u32;
    let byte2 = *code.get((pc + offset + 1) as usize).unwrap() as u32;
    let byte3 = *code.get((pc + offset + 2) as usize).unwrap() as u32;
    let byte4 = *code.get((pc + offset + 3) as usize).unwrap() as u32;

    ((byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4) as i32
}

pub fn get_java_byte_array_string_value(value_array: &Array) -> String {
    let element_values: Vec<u16> = value_array.elements.iter()
        .map(|p| match p {
            &Primitive::Char(ref code) => *code,
            p => panic!("Unexpected primitive: {:?}", p),
        })
        .collect();

    String::from_utf16_lossy(element_values.as_slice())
}

pub fn get_java_string_value(string_instance: &Instance) -> String {
    match string_instance.fields.get("value").unwrap() {
        &Primitive::Arrayref(ref rc_value_array) => get_java_byte_array_string_value(&*rc_value_array.borrow()),
        p => panic!("Unexpected primitive: {:?}", p),
    }
}

pub fn get_instance_field_string_value(class_instance: &Instance, field_name: &str) -> String {
    let rc_string = class_instance.fields.get(field_name).unwrap();
    
    match rc_string {
        &Primitive::Objectref(ref rc_string_instance) => get_java_string_value(&*rc_string_instance.borrow()),
        _ => panic!("Expected to find Instance but found {:?}", rc_string),
    }
}