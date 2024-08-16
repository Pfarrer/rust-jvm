#[macro_use]
extern crate log;

mod array;
mod class_hierarchy;
mod eval;
mod frame;
mod instance;
mod native;
mod primitive;
mod utils;
pub mod vm_mem;
pub mod vm_thread;

use model::prelude::*;
use vm_mem::VmMemImpl;
use vm_thread::VmTheadImpl;

pub fn new_vm(classloader: impl Classloader + 'static) -> Vm {
    Vm {
        classloader: Box::new(native::NativeClassloader {
            classloader: Box::new(classloader),
        }),
        mem: VmMem::new(),
    }
}

pub fn bootstrap_vm(classloader: impl Classloader + 'static) -> Vm {
    let vm = new_vm(classloader);

    VmThread::new(&vm, "vm-init".to_string()).invoke_method(
        &"java/lang/System".to_string(),
        &"initPhase1".to_string(),
        &"()V".to_string(),
        false,
    );

    vm
}
