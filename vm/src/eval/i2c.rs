use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!(
        "i2c: Popped Int {} from stack and push it back as Char",
        value
    );
    frame.stack_push(VmPrimitive::Char(value as u16));

    Some(pc + 1)
}
