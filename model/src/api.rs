use crate::class::JvmClass;
use std::io::Read;

pub trait Parser {
    fn parse<T: Read>(&self, reader: &mut T) -> JvmClass;
}
