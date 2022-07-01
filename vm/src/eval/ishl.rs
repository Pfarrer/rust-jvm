use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();

    let result = value1 << (value2 & 0x1f);

    trace!(
        "ishl: Shifting {} left by {} -> pushing {} to stack",
        value1,
        value2 & 0x1f,
        result
    );
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
