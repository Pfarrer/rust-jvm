use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = code.get(pc as usize).unwrap() - 11;
    trace!("fconst_{}: Pushing constant Float {} to stack", index, index);

    frame.stack_push(Primitive::Float(index as f32));

    Some(pc + 1)
}