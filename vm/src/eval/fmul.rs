use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_float();
    let value1 = frame.stack_pop_float();
    let result = value1 * value2;

    trace!(
        "fmul: Popping two Floats {} and {} from stack and push multiplication result {} back",
        value1,
        value2,
        result
    );

    frame.stack_push(Primitive::Float(result));

    Some(pc + 1)
}
