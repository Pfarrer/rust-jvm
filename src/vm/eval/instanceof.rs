use classfile::Classfile;
use classfile::constants::Constant;
use vm::Frame;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    let constant = class.constants.get(index as usize).unwrap();
    let reference = frame.stack_pop();

    let (name1, name2, value) = match (constant, reference) {
        (&Constant::Class(ref class_path), Primitive::Objectref(ref rc_instance)) => {
            let instance = rc_instance.borrow();
            let value = for_class_instance(class_path, &instance.class_path);

            (class_path.clone(), instance.class_path.clone(), value)
        },
        (&Constant::Class(ref class_path), Primitive::Null) => {
            (class_path.clone(), "null".to_owned(), 0)
        },
        o => panic!("Unexpected constant - reference combination: {:?}", o),
    };

    trace!("instanceof: Checking if {} is instance of {} -> {}", name1, name2, value);
    frame.stack_push(Primitive::Int(value));

    Some(pc + 3)
}

fn for_class_instance(expected_class_path: &String, instance_class_path: &String) -> i32 {
    if expected_class_path.eq(instance_class_path) {
        1
    }
    else {
        panic!("Class hierarchie not implemented!");
    }
}