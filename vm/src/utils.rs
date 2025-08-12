use std::{cell::RefCell, rc::Rc};

use crate::{array::VmArrayImpl, class_hierarchy::HierarchyIterator, instance::VmInstanceImpl, vm_mem::VmStaticPoolImpl, vm_thread::VmTheadImpl};
use itertools::Itertools;
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

fn get_java_bytes_utf16_string_value(bytes: &[VmPrimitive]) -> String {
    let element_values: Vec<u16> = bytes.iter()
        .tuples()
        .map(|(h, l)| match (h, l) {
            (VmPrimitive::Byte(ref hb), VmPrimitive::Byte(ref lb)) => (*hb as u16) << 8 | *lb as u16,
            p => panic!("Unexpected primitives: {:?}", p),
        })
        .collect();

    String::from_utf16_lossy(element_values.as_slice())
}
fn get_java_bytes_latin1_string_value(bytes: &[VmPrimitive]) -> String {
    let element_values: Vec<u8> = bytes.iter()
        .map(|h| match h {
            VmPrimitive::Byte(ref b) => *b,
            p => panic!("Unexpected primitives: {:?}", p),
        })
        .collect();

    String::from_utf8(element_values)
        .expect("Failed to convert Latin-1 bytes to String")
}

pub fn get_java_string_value(string_instance: &VmInstance) -> String {
    let is_latin1 = if let &VmPrimitive::Byte(ref coder_value) = string_instance.fields.get("coder").unwrap() {
        *coder_value == 0
    } else {
        panic!("Unexpected coder field type in string instance: {:?}", string_instance.fields.get("coder"));
    };

    match string_instance.fields.get("value").unwrap() {
        &VmPrimitive::Arrayref(ref rc_value_array) => {
            if is_latin1 {
                get_java_bytes_latin1_string_value(&*rc_value_array.borrow().elements)
            } else {
                // Handle UTF-16 case
                get_java_bytes_utf16_string_value(&*rc_value_array.borrow().elements)
            }
        }
        p => panic!("Unexpected primitive: {:?}", p),
    }
}

pub fn create_java_string(vm_thread: &mut VmThread, string: String) -> Rc<RefCell<VmInstance>> {
    let (bytes, is_latin1) = if string.is_ascii() {
        // Encoding as individual bytes is sufficient for ASCII strings
        (string.into_bytes(), true)
    } else {
        // Non-ASCII strings need to be encoded as UTF-16
        let utf16_iter = string.encode_utf16();
        let mut bytes: Vec<u8> = Vec::with_capacity(utf16_iter.clone().count() * 2);
        for code_unit in utf16_iter {
            bytes.push((code_unit & 0xFF) as u8); // low byte
            bytes.push((code_unit >> 8) as u8); // high byte
        }

        (bytes, false)
    };

    // Allocate a byte[] array (atype 8 = byte) with the exact length
    let mut array = VmArray::new_primitive(bytes.len(), 8);
    for (i, b) in bytes.iter().enumerate() {
        array.elements[i] = VmPrimitive::Byte(*b);
    }
    let rc_array = Rc::new(RefCell::new(array));
    
    // Create a new instance of java/lang/String
    let jvm_class = vm_thread.load_and_clinit_class(&"java/lang/String".to_string());
    let mut instance = VmInstance::new(vm_thread, &jvm_class);

    // Set the `value` field to the byte[] we just created
    instance
        .fields
        .insert("value".to_string(), VmPrimitive::Arrayref(rc_array.clone()));

    // Set the `coder` field to 1 for UTF‑16 or 0 for ASCII/Latin-1
    let coder_value = if is_latin1 { 0 } else { 1 };
    instance
        .fields
        .insert("coder".to_string(), VmPrimitive::Byte(coder_value));
    
    Rc::new(RefCell::new(instance))
}

pub fn find_static_field_value(
    vm_thread: &mut VmThread,
    root_class_path: &String,
    field_name: &String,
) -> VmPrimitive {
    let class_paths: Vec<String> = {
        let hierarchy_iter = HierarchyIterator::hierarchy_iter(vm_thread, root_class_path);
        hierarchy_iter
            .map(|(jvm_class, _, _)| jvm_class.this_class)
            .collect()
    };

    for class_path in class_paths {
        let value_option = vm_thread
            .vm
            .mem
            .static_pool
            .get_class_field(&class_path, &field_name);
        if value_option.is_some() {
            return value_option.unwrap();
        }
    }

    panic!("Static field not found: {}.{}", root_class_path, field_name);
}
