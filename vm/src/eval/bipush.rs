use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let u8_value = *code.get(pc as usize + 1).unwrap();
    let i32_value = unsafe {
        let i8_value: i8 = std::mem::transmute(u8_value);
        i8_value as i32
    };

    trace!("bipush: Pushing Int {} to stack", i32_value);
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Int(i32_value));

    Some(pc + 2)
}
