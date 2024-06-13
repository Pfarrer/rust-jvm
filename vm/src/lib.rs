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

pub fn bootstrap_vm(classloader: impl Classloader + 'static) -> Vm {
    let vm = Vm {
        classloader: Box::new(classloader),
        mem: VmMem::new(),
    };

    vm.spawn_thread("vm-init".to_string()).invoke_method(
        &"java/lang/System".to_string(),
        &"initPhase1".to_string(),
        &"()V".to_string(),
        false,
    );

    vm
}
