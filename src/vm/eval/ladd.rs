use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_long();
    let value1 = frame.stack_pop_long();
    let result = value1 + value2;

    trace!("ladd: Adding {} and {} -> pushing {} to stack", value1, value2, result);
    frame.stack_push(Primitive::Long(result));

    Some(pc + 1)
}
