use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "doPrivileged" => do_privileged(vm, class_path, method_name, method_signature),
        "getStackAccessControlContext" => get_stack_access_control_context(vm, class_path, method_name, method_signature),
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;
fn do_privileged(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // TODO check if privileged to call run()Ljava/lang/Object;

    let class_path = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let class_path = rc_instance.borrow().class_path.clone();
        frame.stack_push(Primitive::Objectref(rc_instance));

        class_path
    };

    utils::invoke_method(vm, &class_path, &"run".to_string(), &"()Ljava/lang/Object;".to_string(), true);
}

/// getStackAccessControlContext()Ljava/security/AccessControlContext;
fn get_stack_access_control_context(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Null);

    trace!("Pushed Null to stack");
}
