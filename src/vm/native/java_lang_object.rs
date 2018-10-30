use vm::Vm;
use vm::primitive::Primitive;
use vm::classloader::Classloader;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "hashCode" => hash_code(vm, class_path, method_name, method_signature), // ()I
        "getClass" => get_class(vm, class_path, method_name, method_signature), // ()Ljava/lang/Class;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

fn hash_code(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();

    warn!("hashCode will always return Int(777)");

    frame.stack_pop();
    frame.stack_push(Primitive::Int(777));
}

/// getClass()Ljava/lang/Class;
fn get_class(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let class_path = {
        let mut frame = vm.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let instance = rc_instance.borrow();

        instance.class_path.clone()
    };

    let rc_class_instance = Classloader::get_class(vm, &class_path);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_class_instance));
}
