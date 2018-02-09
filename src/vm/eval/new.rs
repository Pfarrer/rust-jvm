use std::rc::Rc;
use std::cell::RefCell;

use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;
use vm::primitive::Primitive;
use vm::instance::Instance;
use vm::utils;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);

    match class.constants.get(index as usize).unwrap() {
        &Constant::Class(ref class_path) => {
            let class = vm.load_and_clinit_class(class_path);
            let instance = Instance::new(vm, class);

            trace!("new: {} -> pushing reference to stack", class_path);
            frame.stack_push(Primitive::Objectref(Rc::new(RefCell::new(instance))));
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}