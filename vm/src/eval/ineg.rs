use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let (result, _) = value.overflowing_neg();

    trace!(
        "lneg: Popped Int {} from stack and negate it -> pushing Int {} to stack",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
