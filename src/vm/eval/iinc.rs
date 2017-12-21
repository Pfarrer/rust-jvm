use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = *code.get(pc as usize + 1).unwrap();
    let inc_by = *code.get(pc as usize + 2).unwrap();

    let new_value = frame.locals_get_int(index as usize) + inc_by as i32;
    trace!("iinc: Increment local variable {} by {} -> resulting in {}", index, inc_by, new_value);

    frame.locals_write(index as usize, Primitive::Int(new_value));

    Some(pc + 3)
}
