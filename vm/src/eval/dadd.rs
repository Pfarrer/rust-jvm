use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_double();
    let value1 = frame.stack_pop_double();
    let result = value1 + value2;

    trace!(
        "dadd: Adding Double {} and Double {} -> pushing Double {} to stack",
        value1,
        value2,
        result
    );
    frame.stack_push(VmPrimitive::Double(result));

    Some(pc + 1)
}
