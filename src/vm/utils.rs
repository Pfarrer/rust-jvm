use classfile::Classfile;
use classfile::constants::Constant;
use classfile::attributes;
use classfile::Method;
use vm::Vm;
use vm::Frame;
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

pub fn find_method<'a>(classfile: &'a Classfile, name: &String, signature: &String) -> Option<&'a Method> {
    classfile.methods.iter().find(| &method | {
        let correct_name = match classfile.constants.get(method.name_index).unwrap() {
            &Constant::Utf8(ref val) => name.eq(val),
            _ => panic!("Invalid class file"),
        };

        if !correct_name {
            return false;
        }

        match classfile.constants.get(method.descriptor_index).unwrap() {
            &Constant::Utf8(ref val) => signature.eq(val),
            _ => panic!("Invalid class file"),
        }
    })
}

pub fn find_code<'a>(method: &'a Method) -> Option<&'a attributes::CodeAttribute> {
    for attr in method.attributes.iter() {
        if let &attributes::Attribute::Code(ref code) = attr {
            return Some(code);
        }
    }

    None
}

pub fn invoke_method(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
    let class = vm.load_and_clinit_class(class_path);
    let method = find_method(&class, method_name, method_signature)
        .unwrap_or_else(|| panic!("Method not found: {}.{}{}", class_path, method_name, method_signature));
    let mut frame = Frame::new();

    // Parse signature and move arguments from caller frame to callee frame
    let sig = signature::parse_method(method_signature);
    for i in (1..sig.parameters.len()+1).rev() {
        let arg = parent_frame.stack_pop();

        trace!(" - Write argument no. {} to inner frame: {:?}", i, arg);
        frame.locals_write(i, arg);
    }

    // Push the instance reference to local no. 0
    let this_ref = parent_frame.stack_pop_reference();
    trace!(" - Write 'this' reference to inner frame: {:?}", this_ref);
    frame.locals_write(0, this_ref);

    vm.execute_method(&class, &method, &mut frame, parent_frame);
}

pub fn read_u16_code(code: &Vec<u8>, pc: u16) -> u16 {
    let indexbyte1: u16 = (*code.get((pc+1) as usize).unwrap() as u16) << 8;
    let indexbyte2 = (*code.get((pc+2) as usize).unwrap()) as u16;

    indexbyte1 + indexbyte2
}