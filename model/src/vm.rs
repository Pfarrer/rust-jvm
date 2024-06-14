use crate::prelude::Classloader;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::RwLock};

pub type NativeMethod = fn(thread: &mut VmThread) -> ();

#[derive(Debug)]
pub struct VmFrame {
    pub class_path: String,
    pub method_name: String,
    pub method_signature: String,

    pub locals: Vec<VmPrimitive>,
    pub stack: Vec<VmPrimitive>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VmPrimitive {
    Boolean(bool),

    Byte(u8),

    Short(i16),
    Char(u16),

    Int(i32),
    Float(f32),

    Long(i64),
    Double(f64),

    Arrayref(Rc<RefCell<VmArray>>),
    Objectref(Rc<RefCell<VmInstance>>),
    ReturnAddress(u16),

    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VmArray {
    pub atype: Option<u8>,
    pub class_path: Option<String>,
    pub elements: Vec<VmPrimitive>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VmInstance {
    pub class_path: String,
    pub fields: HashMap<String, VmPrimitive>,
}

pub struct VmStaticPool {
    pub pool: RwLock<HashMap<String, HashMap<String, VmPrimitive>>>,
}
pub struct VmStringPool {
    pub pool: RwLock<HashMap<String, Rc<RefCell<VmInstance>>>>,
}
pub struct VmClassObjectPool {
    pub pool: RwLock<HashMap<String, Rc<RefCell<VmInstance>>>>,
}

pub struct VmMem {
    pub static_pool: VmStaticPool,
    pub string_pool: VmStringPool,

    // Object pool for java/lang/Class instances
    pub class_object_pool: VmClassObjectPool,
}

unsafe impl Send for VmMem {}
unsafe impl Sync for VmMem {}

pub struct VmThread<'a> {
    pub vm: &'a Vm,
    pub thread_name: String,
    pub frame_stack: Vec<VmFrame>,
}

pub struct Vm {
    pub classloader: Box<dyn Classloader>,
    pub mem: VmMem,
}
