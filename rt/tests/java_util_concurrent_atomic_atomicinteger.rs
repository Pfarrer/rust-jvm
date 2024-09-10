use std::{cell::RefCell, rc::Rc};

use ctor::ctor;
use model::prelude::*;
use rstest::rstest;
use pretty_assertions::assert_eq;
use rt::NativeClassloader;
use vm::{frame::VmFrameImpl, instance::VmInstanceImpl, new_vm, vm_thread::VmTheadImpl};

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
fn increment_and_get() {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());
    new_instance(&mut vm_thread, "java/util/concurrent/atomic/AtomicInteger");
    
    vm_thread.invoke_method(
        &"java/util/concurrent/atomic/AtomicInteger".to_string(),
        &"incrementAndGet".to_string(),
        &"()I".to_string(),
        true,
    );

    assert_eq!(vm_thread.frame_stack.last().unwrap().stack.last().unwrap(), &VmPrimitive::Int(1));
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};
    let classloader = NativeClassloader {
        classloader: Box::new(rt::make_classloader(&parser)),
    };

    new_vm(classloader)
}

fn new_instance(vm_thread: &mut VmThread, class_path: &str) {
    let class = vm_thread.load_and_clinit_class(&class_path.to_string());
    let instance = VmInstance::new(vm_thread, &class);
    let rc_instance = Rc::new(RefCell::new(instance));

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(rc_instance.clone()));
    frame.stack_push(VmPrimitive::Objectref(rc_instance));

    vm_thread.invoke_method(
        &class_path.to_string(),
        &"<init>".to_string(),
        &"()V".to_string(),
        true,
    );
}