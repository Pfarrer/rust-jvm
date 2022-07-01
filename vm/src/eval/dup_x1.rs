use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value1 = frame.stack_pop();
    let value2 = frame.stack_pop();

    trace!("dup_x1: Duplicate the top operand stack value and insert two values down");
    frame.stack_push(value1.clone());
    frame.stack_push(value2);
    frame.stack_push(value1);

    Some(pc + 1)
}
