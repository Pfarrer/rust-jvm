use classfile::constants::Constant;
use model::class::*;
use vm::utils;
use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, jvm_class: &JvmClass, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    let count = *code.get((pc + 3) as usize).unwrap() as usize;
    // let _ = *code.get((pc+4) as usize); // Will be always 0

    let root_class_path = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let class_path = match frame.stack_peek_reverse(count - 1) {
            &Primitive::Objectref(ref rc_object) => rc_object.borrow().class_path.clone(),
            p => panic!("Expected to pop Objectref from stack but found: {:?}", p),
        };

        class_path
    };

    match class.constants.get(index as usize).unwrap() {
        &Constant::InterfaceMethodref(ref class_path, ref method_name, ref method_signature) => {
            debug!(
                "invokeinterface: {}.{}{} on class {}",
                class_path, method_name, method_signature, root_class_path
            );
            utils::invoke_method(vm, &root_class_path, method_name, method_signature, true);
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 5)
}
