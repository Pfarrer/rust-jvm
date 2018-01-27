use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_float();
    let value1 = frame.stack_pop_float();
    let result = value1 * value2;

    trace!("fmul: Popping two Floats {} and {} from stack and push multiplication result {} back", value1, value2, result);

    frame.stack_push(Primitive::Float(result));

    Some(pc + 1)
}