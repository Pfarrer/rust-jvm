use std::{cell::RefCell, rc::Rc};

use ctor::ctor;
use model::prelude::*;
use pretty_assertions::assert_eq;
use rstest::rstest;
use rt::NativeClassloader;
use vm::{
    frame::VmFrameImpl, instance::VmInstanceImpl, new_vm, utils::create_java_string,
    vm_thread::VmTheadImpl,
};

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
fn set_and_get() {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());
    let rc_properties_instance = new_instance(&mut vm_thread, "java/util/Properties");

    let expected = create_java_string(&mut vm_thread, "val".to_string());

    {
        let key = create_java_string(&mut vm_thread, "k".to_string());
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_properties_instance.clone()));
        frame.stack_push(VmPrimitive::Objectref(key));
        frame.stack_push(VmPrimitive::Objectref(expected.clone()));
    }
    vm_thread.invoke_method(
        &"java/util/Properties".to_string(),
        &"setProperty".to_string(),
        &"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;".to_string(),
        true,
    );

    {
        let key = create_java_string(&mut vm_thread, "k".to_string());

        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_properties_instance.clone()));
        frame.stack_push(VmPrimitive::Objectref(key));
    }
    vm_thread.invoke_method(
        &"java/util/Properties".to_string(),
        &"getProperty".to_string(),
        &"(Ljava/lang/String;)Ljava/lang/String;".to_string(),
        true,
    );

    assert_eq!(
        vm_thread.frame_stack.last().unwrap().stack.last().unwrap(),
        &VmPrimitive::Objectref(expected)
    );
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};
    let classloader = NativeClassloader {
        classloader: Box::new(rt::make_classloader(&parser)),
    };

    new_vm(classloader)
}

fn new_instance(vm_thread: &mut VmThread, class_path: &str) -> Rc<RefCell<VmInstance>> {
    let class = vm_thread.load_and_clinit_class(&class_path.to_string());
    let instance = VmInstance::new(vm_thread, &class);
    let rc_instance = Rc::new(RefCell::new(instance));

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(rc_instance.clone()));

    vm_thread.invoke_method(
        &class_path.to_string(),
        &"<init>".to_string(),
        &"()V".to_string(),
        true,
    );

    rc_instance
}
