use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = code.get(pc as usize).unwrap() - 3;
    trace!("iconst_{}: Pushing constant Int {} to stack", index, index);

    frame.stack_push(Primitive::Int(index as i32));

    Some(pc + 1)
}