use std::rc::Rc;
use std::cell::RefCell;

use classfile::Classfile;
use classfile::constants::Constant;
use vm::Frame;
use vm::primitive::Primitive;
use vm::array::Array;
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


    trace!("anewarray: Create new Array of length {} and push Arrayref to stack", count);

    let array = Array::new_complex(count as usize, class_path);
    frame.stack_push(Primitive::Arrayref(Rc::new(RefCell::new(array))));

    Some(pc + 3)
}