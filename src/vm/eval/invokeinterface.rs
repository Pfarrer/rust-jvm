use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::utils;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    let count = code.get((pc+3) as usize).unwrap();
    // let _ = *code.get((pc+4) as usize); // Will be always 0

    match class.constants.get(index as usize).unwrap() {
        &Constant::InterfaceMethodref(ref class_path, ref method_name, ref method_signature) => {
            debug!("invokeinterface: {}.{}{}", class_path, method_name, method_signature);
            utils::invoke_method(vm, class_path, method_name, method_signature, true);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 5)
}
