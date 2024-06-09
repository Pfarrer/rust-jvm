#[macro_use]
extern crate log;

mod array;
mod class_hierarchy;
mod eval;
mod frame;
mod instance;
mod primitive;
mod utils;
mod vm_mem;
mod vm_thread;

use crate::primitive::Primitive;
use crate::vm_thread::VmThread;
use model::api::Classloader;
use vm_mem::VmMem;

pub struct Vm {
    classloader: Box<dyn Classloader>,
    mem: VmMem,
}

impl Vm {
    pub fn new(classloader: impl Classloader + 'static) -> Vm {
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

    pub fn spawn_thread(&self, thread_name: String) -> VmThread {
        VmThread::new(self, thread_name)
    }
}
