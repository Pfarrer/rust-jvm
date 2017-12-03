use std::cmp::Ordering;

use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_long();
    let value1 = frame.stack_pop_long();

    let result = match value2.cmp(&value1) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };

    trace!("lcmp: Comparing {} and {} -> pushing {} to stack", value1, value2, result);
    frame.stack_push_int(result);

    Some(pc+1)
}
