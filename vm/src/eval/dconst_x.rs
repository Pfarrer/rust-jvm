use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let value = code.get(pc as usize).unwrap() - 14;

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Double(value as f64));

    trace!(
        "dconst_{}: Pushed constant Double {} to stack",
        value,
        value
    );

    Some(pc + 1)
}
