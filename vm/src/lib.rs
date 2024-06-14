mod array;
mod class_hierarchy;
// mod eval;
mod native;
mod frame;
mod instance;
mod primitive;
mod utils;
mod vm_mem;
mod vm_thread;

use model::prelude::*;
use vm_mem::VmMemImpl;
use vm_thread::VmTheadImpl;

pub fn bootstrap_vm(classloader: impl Classloader + 'static) -> Vm {
    let vm = Vm {
        classloader: Box::new(classloader),
        mem: VmMem::new(),
    };

    VmThread::new(&vm, "vm-init".to_string()).invoke_method(
        &"java/lang/System".to_string(),
        &"initPhase1".to_string(),
        &"()V".to_string(),
        false,
    );

    vm
}
