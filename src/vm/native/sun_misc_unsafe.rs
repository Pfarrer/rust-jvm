use vm::Vm;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => noop(class_path, method_name, method_signature), // ()V
        "objectFieldOffset" => object_field_offset(vm, class_path, method_name, method_signature), // (Ljava/lang/reflect/Field;)J
        "allocateMemory" => allocate_memory(vm, class_path, method_name, method_signature), // (J)J
        "freeMemory" => free_memory(vm, class_path, method_name, method_signature), // (J)V
        "putLong" => put_long(vm, class_path, method_name, method_signature), // (JJ)V
        "getByte" => get_byte(vm, class_path, method_name, method_signature), // (J)B
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// (Ljava/lang/reflect/Field;)J
fn object_field_offset(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // Remove parameter from stack
    let frame = vm.frame_stack.last_mut().unwrap();
    let _ = frame.stack_pop_objectref();
//    let instance = rc_instance.borrow();

    warn!("Not properly implemented -> will always return 0L");

    frame.stack_push(Primitive::Long(0));
}

/// (J)J
fn allocate_memory(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let size = vm.frame_stack.last_mut().unwrap().stack_pop_long();
    let ptr = vm.memory_pool.allocate(size as usize);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Long(ptr as i64));
}

/// (J)V
fn free_memory(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let _ = vm.frame_stack.last_mut().unwrap().stack_pop_long();
    vm.memory_pool.free();
}

/// putLong(JJ)V
fn put_long(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let value = vm.frame_stack.last_mut().unwrap().stack_pop_long();
    let address = vm.frame_stack.last_mut().unwrap().stack_pop_long();

    trace!("Popped two Longs from stack and write value {} at address {}", value, address);

    unsafe { vm.memory_pool.put_long(address as usize, value) };
}

/// getByte(J)B
fn get_byte(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let address = vm.frame_stack.last_mut().unwrap().stack_pop_long();
    let value = unsafe { vm.memory_pool.get_byte(address as usize) };

    trace!("Popped address {} from stack and push byte {} back", address, value);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Byte(value));
}