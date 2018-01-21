use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;
use vm::utils;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Methodref(ref class_path, ref method_name, ref method_signature) => {
            debug!("invokevirtual: {}.{}{}", class_path, method_name, method_signature);
            // TODO Don't really know whether this implementation is correct
            utils::invoke_method(vm, class_path, method_name, method_signature, frame);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}