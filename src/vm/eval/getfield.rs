use classfile::Classfile;
use classfile::constants::Constant;
use vm::Frame;
use vm::utils;

pub fn eval(class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Fieldref(ref class_path, ref field_name, _) => {
            // Pop Objectref from stack
            let rc_instance = frame.stack_pop_objectref();
            let instance = rc_instance.borrow_mut();

            trace!("getfield: Popping Objectref from stack and push value of field {}.{} on stack", class_path, field_name);
            let value = instance.fields.get(field_name).unwrap();
            frame.stack_push(value.clone());
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}