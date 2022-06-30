use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;
use vm::class_hierarchy::ClassHierarchy;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    let constant = class.constants.get(index as usize).unwrap();
    let checkfor_class_name = match constant {
        &Constant::Class(ref class_path) => class_path.clone(),
        o => panic!("Unexpected constant: {:?}", o),
    };

    let instance_class_name = pop_instance_and_get_class_name(vm);
    
    let value = instance_class_name.as_ref()
        .map(|name| for_class_instance(vm, &checkfor_class_name, &name))
        .unwrap_or(0i32);

    trace!("instanceof: Checking if {} is instance of {} -> {}", checkfor_class_name, instance_class_name.unwrap_or("null".to_owned()), value);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Int(value));

    Some(pc + 3)
}

fn pop_instance_and_get_class_name(vm: &mut Vm) -> Option<String> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let reference = frame.stack_pop();
    match reference {
        Primitive::Objectref(ref rc_instance) => Some(rc_instance.borrow().class_path.clone()),
        Primitive::Null => None,
        _ => panic!("Unexpected value, found {:?}", reference),
    }
}

fn for_class_instance(vm: &mut Vm, checkfor_class_name: &String, instance_class_path: &String) -> i32 {
    let hierarchy_iter = ClassHierarchy::hierarchy_iter(vm, &instance_class_path);
    for (class, _, _) in hierarchy_iter {
        let iter_class_name = utils::get_class_path(&class);
        if checkfor_class_name.eq(&iter_class_name) {
            return 1;
        }
    }

    0
}