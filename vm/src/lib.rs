#[macro_use]
extern crate log;

pub mod array;
pub mod class_hierarchy;
pub mod frame;
pub mod instance;
pub mod primitive;
pub mod utils;
pub mod vm_mem;
pub mod vm_thread;
mod eval;

use model::prelude::*;
use vm_mem::VmMemImpl;

pub fn new_vm(classloader: impl Classloader + 'static) -> Vm {
    Vm {
        classloader: Box::new(classloader),
        mem: VmMem::new(),
    }
}
