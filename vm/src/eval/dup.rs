use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    let value = frame.stack_pop();
    trace!("dup: Duplicating last stack element");

    frame.stack_push(value.clone());
    frame.stack_push(value);

    Some(pc + 1)
}
