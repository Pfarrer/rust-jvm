use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();

    let s = value2 & 0x1f;

    let result = if value1 >= 0 {
        value1 >> s
    } else {
        (value1 >> s) + (2 << !s)
    };

    trace!(
        "iushr: Shifting {} right unsigned by {} -> pushing {} to stack",
        value1,
        s,
        result
    );
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
