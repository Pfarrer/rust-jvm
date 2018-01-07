use std::rc::Rc;
use std::cell::RefCell;

use vm::primitive::Primitive;
use vm::array::Array;
use vm::instance::Instance;

#[derive(Debug)]
pub struct Frame {
    locals: Vec<Primitive>,
    stack: Vec<Primitive>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            locals: Vec::new(),
            stack: Vec::new(),
        }
    }

//    pub fn new(max_locals: usize, max_stack: usize) -> Frame {
//        Frame {
//            locals: Vec::with_capacity(max_locals),
//            stack: Vec::with_capacity(max_stack),
//        }
//    }

    pub fn locals_write(&mut self, index: usize, val: Primitive) {
        while self.locals.len() <= index {
            self.locals.push(Primitive::Null);
        }

        self.locals[index] = val
    }
    pub fn stack_push(&mut self, val: Primitive) {
        self.stack.push(val)
    }

    pub fn stack_pop(&mut self) -> Primitive {
        self.stack.pop().unwrap()
    }

//    pub fn stack_pop_byte(&mut self) -> u8 {
//        match self.stack_pop() {
//            Primitive::Byte(v) => v,
//            p => panic!("Expected to pop Byte from stack but found: {:?}", p),
//        }
//    }

    pub fn stack_pop_int(&mut self) -> i32 {
        match self.stack_pop() {
            Primitive::Int(v) => v,
            p => panic!("Expected to pop Int from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_long(&mut self) -> i64 {
        match self.stack_pop() {
            Primitive::Long(v) => v,
            p => panic!("Expected to pop Long from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_arrayref(&mut self) -> Rc<RefCell<Array>> {
        match self.stack_pop() {
            Primitive::Arrayref(rc_array) => rc_array,
            p => panic!("Expected to pop Arrayref from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_objectref(&mut self) -> Rc<RefCell<Instance>> {
        match self.stack_pop() {
            Primitive::Objectref(rc_object) => rc_object,
            p => panic!("Expected to pop Objectref from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_reference(&mut self) -> Primitive {
        let value = self.stack_pop();
        match value {
            Primitive::Arrayref(_) => (),
            Primitive::Objectref(_) => (),
            Primitive::ReturnAddress(_) => (),
            Primitive::Null => (),
            _ => panic!("Popped unexpected value from stack, found: {:?}", value),
        };

        value
    }

    pub fn locals_get(&mut self, index: usize) -> &Primitive {
        self.locals.get(index).unwrap()
    }

    pub fn locals_get_int(&mut self, index: usize) -> i32 {
        match self.locals.get(index).unwrap() {
            &Primitive::Int(ref value) => value.clone(),
            p => panic!("Expected to get Int from locals but found: {:?}", p),
        }
    }

    pub fn locals_get_long(&mut self, index: usize) -> i64 {
        match self.locals.get(index).unwrap() {
            &Primitive::Long(ref value) => value.clone(),
            p => panic!("Expected to get Long from locals but found: {:?}", p),
        }
    }
}