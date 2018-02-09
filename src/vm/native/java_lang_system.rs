extern crate time;

use vm::Vm;
use vm::frame::Frame;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(vm, class_path, method_name, method_signature, frame),
        "currentTimeMillis" => current_time_millis(class_path, method_name, method_signature, frame), // ()J
        "nanoTime" => nano_time(class_path, method_name, method_signature, frame), // ()J
        "initProperties" => init_properties(class_path, method_name, method_signature, frame), // (Ljava/util/Properties;)Ljava/util/Properties;
        "arraycopy" => arraycopy(class_path, method_name, method_signature, frame), // arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String, frame: &mut Frame) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    vm.invoke_static(class_path, &"initializeSystemClass".to_string(), &"()V".to_string(), frame);
}

fn current_time_millis(class_path: &String, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let time_spec = time::get_time();

    // 1459440009.113178
    let millis_float: f64 = time_spec.sec as f64 + (time_spec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    let millis_int = (millis_float * 1000.0) as i64;

    // Push result to stack
    parent_frame.stack_push(Primitive::Long(millis_int));
}

fn nano_time(class_path: &String, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let nano_time = time::precise_time_ns();

    // Push result to stack
    parent_frame.stack_push(Primitive::Long(nano_time as i64));
}

/// java/lang/System.initProperties(Ljava/util/Properties;)Ljava/util/Properties;
fn init_properties(class_path: &String, method_name: &String, method_signature: &String, _frame: &mut Frame) {
    // java.version         <dd>Java version number
//    * <dt>java.vendor          <dd>Java vendor specific string
//        * <dt>java.vendor.url      <dd>Java vendor URL
//        * <dt>java.home            <dd>Java installation directory
//        * <dt>java.class.version   <dd>Java class version number
//        * <dt>java.class.path      <dd>Java classpath
//        * <dt>os.name              <dd>Operating System Name
//        * <dt>os.arch              <dd>Operating System Architecture
//        * <dt>os.version           <dd>Operating System Version
//        * <dt>file.separator       <dd>File separator ("/" on Unix)
//    * <dt>path.separator       <dd>Path separator (":" on Unix)
//    * <dt>line.separator       <dd>Line separator ("\n" on Unix)
//    * <dt>user.name            <dd>User account name
//        * <dt>user.home            <dd>User home directory
//        * <dt>user.dir             <dd>User's current working directory

    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    warn!("This method is not implemented!");

//    let rc_instance = frame.stack_pop_objectref();
//    let instance = rc_instance.borrow_mut();
//
//    trace!("getfield: Popping Objectref from stack and push value of field {}.{} on stack", class_path, field_name);
//    let value = instance.fields.get(field_name).unwrap();
}

/// arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V
fn arraycopy(class_path: &String, method_name: &String, method_signature: &String, frame: &mut Frame) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let length = frame.stack_pop_int() as usize;
    let dest_pos = frame.stack_pop_int() as usize;
    let rc_dest_array = frame.stack_pop_arrayref();
    let src_pos = frame.stack_pop_int() as usize;
    let rc_src_array = frame.stack_pop_arrayref();

    let mut dest_array = rc_dest_array.borrow_mut();
    let src_array = rc_src_array.borrow();

    for i in 0..length {
        dest_array.elements[dest_pos+i] = src_array.elements[src_pos+i].clone();
    }
}