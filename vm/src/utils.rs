use std::{cell::RefCell, rc::Rc};

use crate::{array::VmArrayImpl, class_hierarchy::HierarchyIterator, instance::VmInstanceImpl, vm_thread::VmTheadImpl};
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

pub fn get_java_bytes_as_string_value(bytes: &[VmPrimitive]) -> String {
    let element_values: Vec<u16> = bytes.iter()
        .tuples()
        .map(|(h, l)| match (h, l) {
            (VmPrimitive::Byte(ref hb), VmPrimitive::Byte(ref lb)) => (*hb as u16) << 8 | *lb as u16,
            p => panic!("Unexpected primitives: {:?}", p),
        })
        .collect();

    String::from_utf16_lossy(element_values.as_slice())
}

pub fn get_java_string_value(string_instance: &VmInstance) -> String {
    match string_instance.fields.get("value").unwrap() {
        &VmPrimitive::Arrayref(ref rc_value_array) => {
            get_java_bytes_as_string_value(&*rc_value_array.borrow().elements)
        }
        p => panic!("Unexpected primitive: {:?}", p),
    }
}

pub fn create_java_string(vm_thread: &mut VmThread, string: String) -> Rc<RefCell<VmInstance>> {
    let count = string.encode_utf16().count();
    let mut array = VmArray::new_primitive(count*2, 8);
    for (i, c) in string.encode_utf16().enumerate() {
        array.elements[i*2] = VmPrimitive::Byte((c >> 8) as u8);
        array.elements[i*2+1] = VmPrimitive::Byte(c as u8);
    }
    
    array.elements.iter().map(|a| match a {
        VmPrimitive::Byte(b) => *b,
        _ => todo!()
    }).for_each(|b| {
        print!("{} ", b);
    });
    println!();

    let rc_array = Rc::new(RefCell::new(array));
    let jvm_class = vm_thread.load_and_clinit_class(&"java/lang/String".to_string());
    let mut instance = VmInstance::new(vm_thread, &jvm_class);
    instance
        .fields
        .insert("value".to_string(), VmPrimitive::Arrayref(rc_array));
    instance
        .fields
        .insert("coder".to_string(), VmPrimitive::Byte(1)); // coder = 1 which means UTF16 encoded string

    Rc::new(RefCell::new(instance))
}
