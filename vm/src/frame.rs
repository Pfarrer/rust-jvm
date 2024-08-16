use std::{cell::RefCell, rc::Rc};

use model::prelude::*;

pub trait VmFrameImpl {
    fn new(
        max_locals: u16,
        class_path: String,
        method_name: String,
        method_signature: String,
    ) -> VmFrame;

    fn locals_write(&mut self, index: usize, val: VmPrimitive);
    fn stack_push(&mut self, val: VmPrimitive);
    fn stack_pop(&mut self) -> VmPrimitive;
    fn stack_peek_reverse(&mut self, n: usize) -> &VmPrimitive;
    // fn stack_pop_boolean(&mut self) -> bool;
    fn stack_pop_int(&mut self) -> i32;
    fn stack_pop_long(&mut self) -> i64;
    fn stack_pop_float(&mut self) -> f32;
    fn stack_pop_double(&mut self) -> f64;
    fn stack_pop_arrayref(&mut self) -> Rc<RefCell<VmArray>>;
    fn stack_pop_objectref(&mut self) -> Rc<RefCell<VmInstance>>;
    fn stack_pop_reference(&mut self) -> VmPrimitive;
    fn locals_get_int(&mut self, index: usize) -> i32;
    fn locals_get_float(&mut self, index: usize) -> f32;
    fn locals_get_double(&mut self, index: usize) -> f64;
    fn locals_get_long(&mut self, index: usize) -> i64;
    fn locals_get_returnaddress(&mut self, index: usize) -> u16;
    fn locals_get_reference(&mut self, index: usize) -> &VmPrimitive;
}

impl VmFrameImpl for VmFrame {
    fn new(
        max_locals: u16,
        class_path: String,
        method_name: String,
        method_signature: String,
    ) -> VmFrame {
        let mut locals = Vec::with_capacity(max_locals as usize);
        for _ in 0..max_locals {
            locals.push(VmPrimitive::Null)
        }

        VmFrame {
            class_path,
            method_name,
            method_signature,

            locals,
            stack: Vec::new(),
        }
    }

    fn locals_write(&mut self, index: usize, val: VmPrimitive) {
        while self.locals.len() <= index {
            self.locals.push(VmPrimitive::Null);
        }

        self.locals[index] = val
    }
    fn stack_push(&mut self, val: VmPrimitive) {
        self.stack.push(val)
    }

    fn stack_pop(&mut self) -> VmPrimitive {
        self.stack.pop().unwrap()
    }

    fn stack_peek_reverse(&mut self, n: usize) -> &VmPrimitive {
        &self.stack[self.stack.len() - 1 - n]
    }

    // fn stack_pop_boolean(&mut self) -> bool {
    //     match self.stack_pop() {
    //         VmPrimitive::Boolean(v) => v,
    //         VmPrimitive::Int(v) => v == 1,
    //         p => panic!("Expected to pop Boolean from stack but found: {:?}", p),
    //     }
    // }

    fn stack_pop_int(&mut self) -> i32 {
        match self.stack_pop() {
            VmPrimitive::Int(v) => v,
            VmPrimitive::Boolean(v) => {
                if v {
                    1
                } else {
                    0
                }
            }
            VmPrimitive::Char(v) => v as i32,
            VmPrimitive::Byte(v) => v as i32,
            p => panic!("Expected to pop Int from stack but found: {:?}", p),
        }
    }

    fn stack_pop_long(&mut self) -> i64 {
        match self.stack_pop() {
            VmPrimitive::Long(v) => v,
            p => panic!("Expected to pop Long from stack but found: {:?}", p),
        }
    }

    fn stack_pop_float(&mut self) -> f32 {
        match self.stack_pop() {
            VmPrimitive::Float(v) => v,
            p => panic!("Expected to pop Float from stack but found: {:?}", p),
        }
    }

    fn stack_pop_double(&mut self) -> f64 {
        match self.stack_pop() {
            VmPrimitive::Double(v) => v,
            p => panic!("Expected to pop Double from stack but found: {:?}", p),
        }
    }

    fn stack_pop_arrayref(&mut self) -> Rc<RefCell<VmArray>> {
        match self.stack_pop() {
            VmPrimitive::Arrayref(rc_array) => rc_array,
            p => panic!("Expected to pop Arrayref from stack but found: {:?}", p),
        }
    }

    fn stack_pop_objectref(&mut self) -> Rc<RefCell<VmInstance>> {
        match self.stack_pop() {
            VmPrimitive::Objectref(rc_object) => rc_object,
            p => panic!("Expected to pop Objectref from stack but found: {:?}", p),
        }
    }

    fn stack_pop_reference(&mut self) -> VmPrimitive {
        let value = self.stack_pop();
        match value {
            VmPrimitive::Arrayref(_) => (),
            VmPrimitive::Objectref(_) => (),
            VmPrimitive::ReturnAddress(_) => (),
            VmPrimitive::Null => (),
            _ => panic!("Popped unexpected value from stack, found: {:?}", value),
        };

        value
    }

    fn locals_get_int(&mut self, index: usize) -> i32 {
        match self.locals.get(index).unwrap() {
            &VmPrimitive::Int(ref value) => value.clone(),
            p => panic!("Expected to get Int from locals but found: {:?}", p),
        }
    }

    fn locals_get_float(&mut self, index: usize) -> f32 {
        match self.locals.get(index).unwrap() {
            &VmPrimitive::Float(ref value) => value.clone(),
            p => panic!("Expected to get Float from locals but found: {:?}", p),
        }
    }

    fn locals_get_double(&mut self, index: usize) -> f64 {
        match self.locals.get(index).unwrap() {
            &VmPrimitive::Double(ref value) => value.clone(),
            p => panic!("Expected to get Double from locals but found: {:?}", p),
        }
    }

    fn locals_get_long(&mut self, index: usize) -> i64 {
        match self.locals.get(index).unwrap() {
            &VmPrimitive::Long(ref value) => value.clone(),
            p => panic!("Expected to get Long from locals but found: {:?}", p),
        }
    }

    fn locals_get_returnaddress(&mut self, index: usize) -> u16 {
        match self.locals.get(index).unwrap() {
            &VmPrimitive::ReturnAddress(ref address) => address.clone(),
            p => panic!(
                "Expected to get ReturnAddress from locals but found: {:?}",
                p
            ),
        }
    }

    fn locals_get_reference(&mut self, index: usize) -> &VmPrimitive {
        let value = self.locals.get(index).unwrap();
        match value {
            &VmPrimitive::Arrayref(_) => (),
            &VmPrimitive::Objectref(_) => (),
            &VmPrimitive::Null => (),
            _ => panic!(
                "Expected to get reference from locals but found: {:?}",
                value
            ),
        };

        value
    }
}
