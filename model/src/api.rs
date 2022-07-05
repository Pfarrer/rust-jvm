use crate::class::JvmClass;
use std::io::Read;

pub trait Parser {
    fn parse<T: Read>(&self, reader: &mut T) -> JvmClass;
}

pub trait Classloader {
    fn list_classes(&self) -> Vec<&str>;
    fn get_class(&self, classpath: &str) -> Option<&JvmClass>;
}
