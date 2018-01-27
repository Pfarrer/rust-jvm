use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop_int();

    trace!("i2f: Popping Int {} from stack and push it back as Float", value);
    frame.stack_push(Primitive::Float(value as f32));

    Some(pc + 1)
}