use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!(
        "i2l: Popped Int {} from stack and push it back as Long",
        value
    );
    frame.stack_push(VmPrimitive::Long(value as i64));

    Some(pc + 1)
}
