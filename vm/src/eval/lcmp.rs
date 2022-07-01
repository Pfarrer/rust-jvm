use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_long();
    let value1 = frame.stack_pop_long();
    let result = if value1 > value2 {
        -1
    } else if value2 > value1 {
        1
    } else {
        0
    };

    trace!("lcmp: Compared {value1} and {value2} -> pushing {result} to stack");
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
