use vm::primitive::Primitive;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_reference();
    match value {
        Primitive::Null => {
            trace!("ifnonnull: Popped Null from stack -> not branching");

            Some(pc + 3)
        }
        _ => {
            trace!("ifnonnull: Popped Reference from stack -> branching");

            let branchoffset = utils::read_i16_code(code, pc);
            let target_pc: u16 = (pc as i16 + branchoffset) as u16;

            Some(target_pc)
        }
    }
}
