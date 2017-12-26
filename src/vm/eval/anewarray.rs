use classfile::Classfile;
use classfile::constants::Constant;
use vm::Frame;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let count = frame.stack_pop_int();
    let index = utils::read_u16_code(code, pc);

    if count < 0 {
        panic!("Not implemented: Throw NegativeArraySizeException");
    }

    let class_path = match class.constants.get(index as usize).unwrap() {
        &Constant::Class(ref class_path) => class_path.clone(),
        c => panic!("Unexpected constant ref: {:?}", c),
    };

    let default_value = Primitive::Null;

    let mut array = Vec::with_capacity(count as usize);
    for _ in 0..count {
        array.push(default_value.clone());
    }

    trace!("anewarray: Create new Array of length {} and push Arrayref to stack", count);

    frame.stack_push(Primitive::Arrayref(0, Box::new(array)));

    Some(pc + 3)
}