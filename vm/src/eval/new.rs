use model::class_constant_impl::ClassConstantAccessor;
use model::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::instance::VmInstance;
use crate::primitive::VmPrimitive;
use crate::utils;
use crate::vm_thread::VmThread;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc) as usize;

    let class_path = jvm_class.constants.get_class_or(index).unwrap();
    let class = vm_thread.load_and_clinit_class(class_path);
    let instance = VmInstance::new(vm_thread, &class);

    trace!("new: {} -> Pushing reference to stack", class_path);
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(Rc::new(RefCell::new(instance))));

    Some(pc + 3)
}
