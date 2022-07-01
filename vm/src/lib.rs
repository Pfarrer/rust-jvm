#[macro_use]
extern crate log;

mod array;
mod class_hierarchy;
mod eval;
mod frame;
mod instance;
mod mem;
mod primitive;
mod utils;
mod vm_thread;

use crate::primitive::Primitive;
use crate::vm_thread::VmThread;
use loader::Classloader;
use mem::VmMem;
use std::sync::RwLock;

pub struct Vm {
    classloader: Box<dyn Classloader>,
    mem: RwLock<VmMem>,
}

impl Vm {
    pub fn new(classloader: impl Classloader + 'static) -> Vm {
        Vm {
            classloader: Box::new(classloader),
            mem: RwLock::new(VmMem::new()),
        }
    }

    pub fn spawn_thread(&self, thread_name: String) -> VmThread {
        VmThread::new(self, thread_name)
    }
}
