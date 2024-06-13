use crate::class_hierarchy::HierarchyIterator;
use crate::utils;
use crate::{VmPrimitive, VmThread};
use model::prelude::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    let constant = jvm_class.constants.get(index as usize).unwrap();
    let checkfor_class_name = match constant {
        &ClassConstant::Class(ref class_path) => class_path.clone(),
        o => panic!("Unexpected constant: {:?}", o),
    };

    let instance_class_name = pop_instance_and_get_class_name(vm_thread);

    let value = instance_class_name
        .as_ref()
        .map(|name| for_class_instance(vm_thread, &checkfor_class_name, &name))
        .unwrap_or(0i32);

    trace!(
        "instanceof: Checking if {} is instance of {} -> {}",
        checkfor_class_name,
        instance_class_name.unwrap_or("null".to_owned()),
        value
    );

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Int(value));

    Some(pc + 3)
}

fn pop_instance_and_get_class_name(vm_thread: &mut VmThread) -> Option<String> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let reference = frame.stack_pop();
    match reference {
        VmPrimitive::Objectref(ref rc_instance) => Some(rc_instance.borrow().class_path.clone()),
        VmPrimitive::Null => None,
        _ => panic!("Unexpected value, found {:?}", reference),
    }
}

fn for_class_instance(
    vm_thread: &mut VmThread,
    checkfor_class_name: &String,
    instance_class_path: &String,
) -> i32 {
    let hierarchy_iter = HierarchyIterator::hierarchy_iter(vm_thread, &instance_class_path);
    for (class, _, _) in hierarchy_iter {
        if checkfor_class_name.eq(&class.this_class) {
            return 1;
        }
    }

    0
}
