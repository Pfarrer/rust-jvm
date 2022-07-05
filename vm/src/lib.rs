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
        Vm {
            classloader: Box::new(classloader),
            mem: VmMem::new(),
        }
    }

    pub fn spawn_thread(&self, thread_name: String) -> VmThread {
        VmThread::new(self, thread_name)
    }
}
