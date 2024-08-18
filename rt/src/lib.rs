use std::path::PathBuf;

use loader::ClassfileLoader;
use model::prelude::*;
use vm::{new_vm, vm_thread::VmTheadImpl};

mod native;

pub use native::NativeClassloader;

pub fn make_classloader(parser: &impl Parser) -> impl Classloader {
    let rt_path = PathBuf::from("/home/brian/Code/rust-jvm/rt/jmods/java.base/classes");

    ClassfileLoader::open(rt_path, parser).unwrap()
}


pub fn bootstrap_vm(classloader: impl Classloader + 'static) -> Vm {
    let vm = new_vm(native::NativeClassloader {
        classloader: Box::new(classloader),
    });

    VmThread::new(&vm, "vm-init".to_string()).invoke_method(
        &"java/lang/System".to_string(),
        &"initPhase1".to_string(),
        &"()V".to_string(),
        false,
    );

    vm
}

// pub fn initialize() {
//     VmThread::new(&vm, "vm-init".to_string()).invoke_method(
//         &"java/lang/System".to_string(),
//         &"initPhase1".to_string(),
//         &"()V".to_string(),
//         false,
//     );
// }