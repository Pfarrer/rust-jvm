use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let value = code.get(pc as usize).unwrap() - 11;

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Float(value as f32));

    trace!("fconst_{}: Pushed constant Float {} to stack", value, value);

    Some(pc + 1)
}
