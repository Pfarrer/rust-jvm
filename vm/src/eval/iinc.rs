use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = *code.get(pc as usize + 1).unwrap();
    let inc_by = *code.get(pc as usize + 2).unwrap() as i8;

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let old_value = frame.locals_get_int(index as usize);
    let new_value = old_value + inc_by as i32;
    trace!(
        "iinc: Increment local variable {} = {} by {} -> resulting in {}",
        index,
        old_value,
        inc_by,
        new_value
    );

    frame.locals_write(index as usize, VmPrimitive::Int(new_value));

    Some(pc + 3)
}
