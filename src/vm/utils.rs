use classfile;
use classfile::Classfile;
use classfile::constants::Constant;
use classfile::attributes;
use classfile::Method;
use vm::Vm;
use vm::native;
use vm::Frame;
use vm::class_hierarchy::ClassHierarchy;
use vm::primitive::Primitive;
use vm::instance::Instance;
use vm::signature;

pub fn get_utf8_value(classfile: &Classfile, index: usize) -> String {
    match classfile.constants.get(index).unwrap() {
        &Constant::Utf8(ref val) => val.clone(),
        it => panic!("Expected Utf8 but found: {:?}", it),
    }
}

pub fn get_class_path(classfile: &Classfile) -> String {
    match classfile.constants.get(classfile.class_info.this_class as usize).unwrap() {
        &Constant::Class(ref path) => path.clone(),
        it => panic!("Expected Class but found: {:?}", it),
    }
}

pub fn find_method(vm: &mut Vm, root_class_path: &String, name: &String, signature: &String) -> (Classfile, Method) {
    let class = vm.load_and_clinit_class(root_class_path);
    match find_method_in_classfile(&class, name, signature) {
        Some(method) => (class, method),
        None => {
            // Loop through class hierarchy
            let hierarchy_iter = ClassHierarchy::hierarchy_iter(vm, root_class_path);
            for (parent_class, _, _) in hierarchy_iter {
                match find_method_in_classfile(&parent_class, name, signature) {
                    Some(method) => return (parent_class, method),
                    None => (),
                };
            }

            panic!("Method not found: {}.{}{}", root_class_path, name, signature);
        }
    }
}

pub fn find_method_in_classfile(classfile: &Classfile, name: &String, signature: &String) -> Option<Method> {
    classfile.methods.iter().find(| &method | {
        let correct_name = match classfile.constants.get(method.name_index).unwrap() {
            &Constant::Utf8(ref val) => name.eq(val),
            _ => panic!("Invalid class file"),
        };

        if !correct_name {
            return false;
        }

        let correct_signature = match classfile.constants.get(method.descriptor_index).unwrap() {
            &Constant::Utf8(ref val) => signature.eq(val),
            _ => panic!("Invalid class file"),
        };

        if !correct_signature {
            return false;
        }

        // Make sure, method is not abstract
        method.access_flags & classfile::ACC_ABSTRACT == 0
    }).map(|m| m.clone())
}

pub fn find_code<'a>(method: &'a Method) -> Option<&'a attributes::CodeAttribute> {
    for attr in method.attributes.iter() {
        if let &attributes::Attribute::Code(ref code) = attr {
            return Some(code);
        }
    }

    None
}

pub fn get_type_signature(classfile: &Classfile, index: usize) -> signature::TypeSignature {
    match classfile.constants.get(index).unwrap() {
//        &Constant::Fieldref(_, _, ref type_descriptor) => signature::parse_field(type_descriptor),
        &Constant::Utf8(ref type_descriptor) => signature::parse_field(type_descriptor),
        it => panic!("Expected Utf8 but found: {:?}", it),
    }
}

pub fn invoke_method(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
    let (class, method) = find_method(vm, class_path, method_name, method_signature);

    if method.access_flags & classfile::ACC_NATIVE > 0 {
        native::invoke(vm, parent_frame, &class, &method, class_path, method_name, method_signature);
    }
    else {
        let mut frame = Frame::new(class_path.clone(), method_name.clone(), method_signature.clone());

        // Parse signature and move arguments from caller frame to callee frame
        let sig = signature::parse_method(method_signature);
        for i in (1..sig.parameters.len() + 1).rev() {
            let arg = parent_frame.stack_pop();

            //        trace!(" - Write argument no. {} to inner frame: {:?}", i, arg);
            frame.locals_write(i, arg);
        }

        // Push the instance reference to local no. 0
        let this_ref = parent_frame.stack_pop_reference();
        //    trace!(" - Write 'this' reference to inner frame: {:?}", this_ref);
        frame.locals_write(0, this_ref);

        vm.execute_method(&class, &method, &mut frame, parent_frame);
    }
}

pub fn invoke_instance_method(vm: &mut Vm, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
    let rc_instance = parent_frame.stack_pop_objectref();
    let instance = rc_instance.borrow();
    parent_frame.stack_push(Primitive::Objectref(rc_instance.clone()));

    invoke_method(vm, &instance.class_path, method_name, method_signature, parent_frame);
}

pub fn read_u16_code(code: &Vec<u8>, pc: u16) -> u16 {
    let indexbyte1: u16 = (*code.get((pc+1) as usize).unwrap() as u16) << 8;
    let indexbyte2 = (*code.get((pc+2) as usize).unwrap()) as u16;

    indexbyte1 + indexbyte2
}

pub fn get_java_string_value(string_instance: &Instance) -> String {
    match string_instance.fields.get("value").unwrap() {
        &Primitive::Arrayref(ref rc_value_array) => {
            let value_array = rc_value_array.borrow();

            let element_values: Vec<u16> = value_array.elements.iter()
                .map(|p| match p {
                    &Primitive::Char(ref code) => *code,
                    p => panic!("Unexpected primitive: {:?}", p),
                })
                .collect();

            String::from_utf16_lossy(element_values.as_slice())
        },
        p => panic!("Unexpected primitive: {:?}", p),
    }
}