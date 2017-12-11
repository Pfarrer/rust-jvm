use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = code.get(pc as usize).unwrap() - 42;
    trace!("aload_{}: Pushing Reference to stack", index);

    let value = frame.locals_get_reference(0).clone();
    frame.stack_push(Primitive::Reference(value));

    Some(pc + 1)
}