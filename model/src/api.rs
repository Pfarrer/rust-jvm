use anyhow::Result;

use crate::{
    class::{ClassMethod, JvmClass},
    vm::NativeMethod,
};
use std::io::Read;

pub trait Parser {
    fn parse<T: Read>(&self, reader: &mut T) -> Result<JvmClass>;
}

pub trait Classloader {
    fn list_classes(&self) -> Vec<&str>;
    fn get_class(&self, classpath: &str) -> Option<&JvmClass>;
    fn get_native_method(
        &self,
        _jvm_class: &JvmClass,
        _class_method: &ClassMethod,
    ) -> Option<NativeMethod> {
        None
    }
}
