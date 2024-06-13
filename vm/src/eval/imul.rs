use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();
    let result = value1.wrapping_mul(value2);

    trace!(
        "imul: Popping two Ints {} and {} from stack and push result {} back",
        value1,
        value2,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
