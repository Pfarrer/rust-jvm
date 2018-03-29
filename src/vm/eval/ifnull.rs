use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_reference();
    match value {
        Primitive::Null => {
            trace!("ifnull: Popped Null from stack -> branching");

            let branchoffset = utils::read_u16_code(code, pc);
            Some(pc + branchoffset)
        },
        _ => {
            trace!("ifnull: Popped Reference from stack -> not branching");

            Some(pc + 3)
        }
    }
}