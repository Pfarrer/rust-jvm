use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Sign-extend to i32
    let value = *code.get(pc as usize + 1).unwrap() as i32;

    trace!("bipush: Pushing Int {} to stack", value);
    frame.stack_push(Primitive::Int(value));

    Some(pc + 2)
}
