use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop_int();

    trace!("i2l: Popping Int {} from stack and push it back as Long", value);
    frame.stack_push(Primitive::Long(value as i64));

    Some(pc + 1)
}