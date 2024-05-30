use crate::class_hierarchy::HierarchyIterator;
use crate::{utils, Primitive, VmThread};
use model::class::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match jvm_class.constants.0.get(index as usize).unwrap() {
        &ClassConstant::Fieldref(ref class_path, ref field_name, ref type_name) => {
            let value = find_static_value(vm_thread, class_path, field_name);
            trace!(
                "getstatic: {}.{}{} -> push value to stack",
                class_path,
                field_name,
                type_name
            );

            let frame = vm_thread.frame_stack.last_mut().unwrap();
            frame.stack_push(value);
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}

fn find_static_value(
    vm_thread: &mut VmThread,
    root_class_path: &String,
    field_name: &String,
) -> Primitive {
    let class_paths: Vec<String> = {
        let hierarchy_iter = HierarchyIterator::hierarchy_iter(vm_thread, root_class_path);
        hierarchy_iter
            .map(|(jvm_class, _, _)| jvm_class.this_class)
            .collect()
    };

    for class_path in class_paths {
        let value_option = vm_thread
            .vm
            .mem
            .static_pool
            .get_class_field(&class_path, &field_name);
        if value_option.is_some() {
            return value_option.unwrap();
        }
    }

    panic!("Static field not found: {}.{}", root_class_path, field_name);
}
