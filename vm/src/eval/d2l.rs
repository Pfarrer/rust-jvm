use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_double();
    let result = value as i64;

    trace!(
        "d2l: Popped Double {} from stack and push it back as Long {}",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Long(value as i64));

    Some(pc + 1)
}
