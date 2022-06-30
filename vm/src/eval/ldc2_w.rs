use classfile::constants::Constant;
use classfile::Classfile;
use vm::primitive::Primitive;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();

    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Long(ref value) => {
            trace!("ldc2_w: Pushing Long {} to stack", value);
            frame.stack_push(Primitive::Long(value.clone()));
        }
        &Constant::Double(ref value) => {
            trace!("ldc2_w: Pushing Double {} to stack", value);
            frame.stack_push(Primitive::Double(value.clone()));
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}
