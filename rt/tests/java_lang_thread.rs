use std::path::PathBuf;
use ctor::ctor;
use model::prelude::*;
use pretty_assertions::assert_eq;
use rstest::rstest;
use loader::{ClassfileLoader, CompositeLoader};
use rt::NativeClassloader;
use vm::{new_vm, vm_thread::VmTheadImpl};
use vm::utils::create_java_string;

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
fn issue21_thread_start_join() {
    let vm = new_test_vm();
    let mut vm_thread = VmThread::new(&vm, "test".to_string());

    vm_thread.invoke_method(
        &"java_lang_thread/Issue21".to_string(),
        &"test".to_string(),
        &"()Ljava/lang/String;".to_string(),
        false,
    );

    assert_eq!(
        vm_thread.frame_stack.last().unwrap().stack.last().unwrap(),
        &VmPrimitive::Null // TODO String will be returned create_java_string("Thread-0: initial\nThread-0: started\nThread-0: done")
        // &VmPrimitive::Objectref(expected)
    );
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};

    let runtime_classloader = Box::new(rt::make_classloader(&parser));
    let test_classes_classloader = {
        let path_str = std::env::var("JAVA_TEST_CLASSES").unwrap();
        let path = PathBuf::from(path_str);

        Box::new(ClassfileLoader::open(path, &parser).unwrap())
    };

    let classloader = NativeClassloader {
        classloader: Box::new(CompositeLoader::open(vec![runtime_classloader, test_classes_classloader])),
    };

    new_vm(classloader)
}
