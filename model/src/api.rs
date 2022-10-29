use crate::class::{ClassMethod, JvmClass};
use std::io::Read;

pub trait Parser {
    fn parse<T: Read>(&self, reader: &mut T) -> JvmClass;
}

pub type NativeMethod = fn() -> ();

pub trait Classloader {
    fn list_classes(&self) -> Vec<&str>;
    fn get_class(&self, classpath: &str) -> Option<&JvmClass>;
    fn get_native_method(
        &self,
        jvm_class: &JvmClass,
        class_method: &ClassMethod,
    ) -> Option<NativeMethod> {
        None
    }
}
