use classfile::constants::Constant;
use classfile::Classfile;
use vm::classloader::Classloader;
use vm::primitive::Primitive;
use vm::string_pool::StringPool;
use vm::utils;
use vm::Vm;

/// Can handle instructions ldc (decimal 18) and ldc_2 (decimal 19).
pub fn eval(vm: &Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Check which instruction triggered this call, if it was ldc, then only one byte should be read,
    // when it was ldc_w, two bytes must be read
    let (index, pc_inc, instr_name) = match *code.get(pc as usize).unwrap() {
        18 => (*code.get((pc + 1) as usize).unwrap() as u16, 2, "ldc"),
        19 => (utils::read_u16_code(code, pc), 3, "ldc_w"),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    match class.constants.get(index as usize).unwrap() {
        &Constant::String(ref value) => {
            trace!("{}: Pushing String \"{}\" to stack", instr_name, value);

            let rc_instance = StringPool::intern(vm, value);
            vm.frame_stack
                .last_mut()
                .unwrap()
                .stack_push(Primitive::Objectref(rc_instance));
        }
        &Constant::Float(ref value) => {
            trace!("{}: Pushing Float {} to stack", instr_name, value);
            vm.frame_stack
                .last_mut()
                .unwrap()
                .stack_push(Primitive::Float(value.clone()));
        }
        &Constant::Integer(ref value) => {
            trace!("{}: Pushing Int {} to stack", instr_name, value);
            vm.frame_stack
                .last_mut()
                .unwrap()
                .stack_push(Primitive::Int(value.clone()));
        }
        &Constant::Class(ref class_path) => {
            trace!("{}: Found Class {}", instr_name, class_path);
            let rc_instance = Classloader::get_class(vm, class_path);
            vm.frame_stack
                .last_mut()
                .unwrap()
                .stack_push(Primitive::Objectref(rc_instance));
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + pc_inc)
}
