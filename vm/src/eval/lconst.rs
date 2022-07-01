use crate::{Primitive, VmThread};

pub fn eval(val: i64, vm: &Vm, pc: u16) -> Option<u16> {
    trace!("lconst_{}: Pushing {}L to stack", val, val);
    vm.frame_stack
        .last_mut()
        .unwrap()
        .stack_push(Primitive::Long(val));

    Some(pc + 1)
}
