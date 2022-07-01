use crate::{Primitive, utils, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop();
    let value1 = frame.stack_pop();

    let equals = match (&value1, &value2) {
        (&Primitive::Objectref(ref rc_instance1), &Primitive::Objectref(ref rc_instance2)) => rc_instance1.eq(rc_instance2),
        (&Primitive::Objectref(_), &Primitive::Null) => false,
        (&Primitive::Null, &Primitive::Objectref(_)) => false,
        (&Primitive::Null, &Primitive::Null) => true,
        _ => panic!("Not implemented: {:?} -- {:?}", value1, value2),
    };

    let (cmp_result, instr_name) = match *code.get(pc as usize).unwrap() {
        165 => (equals, "eq"),
        166 => (!equals, "ne"),
        _ => panic!("if_acmp_x::eval was called on a non if_acmp_x instruction."),
    };

    trace!("if_acmp{}: Popped two references from stack, compare result: {}? -> {}", instr_name, instr_name, cmp_result);

    if cmp_result {
        let branch_offset = utils::read_i16_code(code, pc);
        let target_pc: u16 = (pc as i16 + branch_offset) as u16;

        Some(target_pc)
    } else {
        Some(pc + 3)
    }
}
