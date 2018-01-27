use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop_float();
    let result = value as i32;

    trace!("i2f: Popping Float {} from stack and push it back as Int {}", value, result);
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}