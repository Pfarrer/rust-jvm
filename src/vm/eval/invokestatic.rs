use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let indexbyte1: u16 = (*code.get((pc+1) as usize).unwrap() as u16) << 8;
    let indexbyte2: u16 = (*code.get((pc+2) as usize).unwrap()) as u16;
    let index = indexbyte1 + indexbyte2;

    match class.constants.get(index as usize).unwrap() {
        &Constant::Methodref(ref class_name, ref method_name, ref method_signature) => {
            trace!("invokestatic: {}.{}{}", class_name, method_name, method_signature);
            vm.invoke_static(class_name, method_name, method_signature, frame)
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc+3)
}