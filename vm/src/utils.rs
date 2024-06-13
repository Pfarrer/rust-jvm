use crate::class_hierarchy::HierarchyIterator;
use model::prelude::*;

pub fn find_method(
    thread: &mut VmThread,
    root_class_path: &String,
    name: &String,
    signature: &String,
) -> (JvmClass, ClassMethod) {
    // Loop through class hierarchy
    let hierarchy_iter = HierarchyIterator::hierarchy_iter(thread, root_class_path);
    for (class, _, _) in hierarchy_iter {
        match find_method_in_classfile(&class, name, signature) {
            Some(method) => return (class, method),
            None => (),
        };
    }

    panic!(
        "ClassMethod not found: {}.{}{}",
        root_class_path, name, signature
    );
}

pub fn find_method_in_classfile(
    jvm_class: &JvmClass,
    name: &str,
    signature_str: &str,
) -> Option<ClassMethod> {
    jvm_class
        .methods
        .iter()
        .find(|&method| {
            if name != method.name || signature_str != format!("{}", method.descriptor) {
                return false;
            }

            // Make sure, method is not abstract
            !method.access_flags.contains(MethodAccessFlag::Abstract)
        })
        .map(|m| m.clone())
}

pub fn find_code(method: &ClassMethod) -> Option<&Code> {
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

pub fn get_java_byte_array_string_value(value_array: &VmArray) -> String {
    let element_values: Vec<u16> = value_array
        .elements
        .iter()
        .map(|p| match p {
            &VmPrimitive::Char(ref code) => *code,
            p => panic!("Unexpected primitive: {:?}", p),
        })
        .collect();

    String::from_utf16_lossy(element_values.as_slice())
}

pub fn get_java_string_value(string_instance: &VmInstance) -> String {
    match string_instance.fields.get("value").unwrap() {
        &VmPrimitive::Arrayref(ref rc_value_array) => {
            get_java_byte_array_string_value(&*rc_value_array.borrow())
        }
        p => panic!("Unexpected primitive: {:?}", p),
    }
}

pub fn get_instance_field_string_value(class_instance: &VmInstance, field_name: &str) -> String {
    let rc_string = class_instance.fields.get(field_name).unwrap();
    match rc_string {
        &VmPrimitive::Objectref(ref rc_string_instance) => {
            get_java_string_value(&*rc_string_instance.borrow())
        }
        _ => panic!("Expected to find VmInstance but found {:?}", rc_string),
    }
}
