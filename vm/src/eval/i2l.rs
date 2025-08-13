use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let result: i64 = value as i64;

    trace!(
        "i2l: Popped Int {} from stack and push it back as Long {}",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Long(result));

    Some(pc + 1)
}
