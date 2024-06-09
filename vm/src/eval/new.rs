use classfile::constants::Constant;
use model::class::*;
use std::cell::RefCell;
use std::rc::Rc;
use vm::instance::Instance;
use vm::utils;
use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, jvm_class: &JvmClass, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);

    match jvm_class.constants.get(index as usize).unwrap() {
        &ClassConstant::Class(ref class_path) => {
            let class = vm.load_and_clinit_class(class_path);
            let instance = Instance::new(vm, class);

            trace!("new: {} -> Pushing reference to stack", class_path);
            let frame = vm_thread.frame_stack.last_mut().unwrap();
            frame.stack_push(Primitive::Objectref(Rc::new(RefCell::new(instance))));
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}
