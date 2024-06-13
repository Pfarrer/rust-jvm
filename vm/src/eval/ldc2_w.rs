use crate::{utils, VmPrimitive, VmThread};
use model::prelude::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    let index = utils::read_u16_code(code, pc);
    match jvm_class.constants.get(index as usize).unwrap() {
        &ClassConstant::Long(ref value) => {
            trace!("ldc2_w: Pushing Long {} to stack", value);
            frame.stack_push(VmPrimitive::Long(value.clone()));
        }
        &ClassConstant::Double(ref value) => {
            trace!("ldc2_w: Pushing Double {} to stack", value);
            frame.stack_push(VmPrimitive::Double(value.clone()));
        }
        it => panic!("Unexpected ClassConstant: {:?}", it),
    };

    Some(pc + 3)
}
