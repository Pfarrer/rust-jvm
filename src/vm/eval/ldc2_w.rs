use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Long(ref value) => {
            trace!("ldc2_w: Pushing Long {} to stack", value);
            frame.stack_push(Primitive::Long(value.clone()));
        },
        &Constant::Double(ref value) => {
            trace!("ldc2_w: Pushing Double {} to stack", value);
            frame.stack_push(Primitive::Double(value.clone()));
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}