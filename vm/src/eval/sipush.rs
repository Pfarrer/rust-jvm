use crate::utils;
use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let value = utils::read_u16_code(code, pc) as i32;

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Int(value));

    trace!("sipush: Pushed Int {} to stack", value);

    Some(pc + 3)
}
