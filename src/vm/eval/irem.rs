use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();
    let result = value1 - (value1 / value2) * value2;

    trace!("irem: Popped Int {} and Int {} from stack, pushing Int {} back", value1, value2, result);
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
