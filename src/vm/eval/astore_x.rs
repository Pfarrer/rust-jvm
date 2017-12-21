use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = code.get(pc as usize).unwrap() - 75;
    trace!("astore_{}: Popping Reference from stack and write to locals", index);

    let value = frame.stack_pop_reference();
    frame.locals_write(index as usize, value);

    Some(pc + 1)
}