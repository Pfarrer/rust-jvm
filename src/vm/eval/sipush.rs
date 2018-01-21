use vm::Frame;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = utils::read_u16_code(code, pc) as i32;

    trace!("sipush: Pushing Int {} to stack", value);
    frame.stack_push(Primitive::Int(value));

    Some(pc + 3)
}
