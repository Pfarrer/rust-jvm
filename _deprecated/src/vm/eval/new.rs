use classfile::Classfile;
use classfile::constants::Constant;
use std::cell::RefCell;
use std::rc::Rc;
use vm::instance::Instance;
use vm::primitive::Primitive;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);

    match class.constants.get(index as usize).unwrap() {
        &Constant::Class(ref class_path) => {
            let class = vm.load_and_clinit_class(class_path);
            let instance = Instance::new(vm, class);

            trace!("new: {} -> Pushing reference to stack", class_path);
            let frame = vm.frame_stack.last_mut().unwrap();
            frame.stack_push(Primitive::Objectref(Rc::new(RefCell::new(instance))));
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}