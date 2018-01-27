use vm::Frame;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
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