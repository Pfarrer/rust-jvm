use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_long();
    let value1 = frame.stack_pop_long();
    let result = if value1 > value2 { -1 } else if value2 > value1 { 1 } else { 0 };

    trace!("lcmp: Comparing {} and {} -> pushing {} to stack", value1, value2, result);
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
