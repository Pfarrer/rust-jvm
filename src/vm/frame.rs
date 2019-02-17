use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use vm::primitive::Primitive;
use vm::array::Array;
use vm::instance::Instance;

#[derive(Debug)]
pub struct Frame {
    pub class_path: String,
    pub method_name: String,
    pub method_signature: String,

    pub locals: Vec<Primitive>,
    pub stack: Vec<Primitive>,
}

impl Frame {
    pub fn new(max_locals: u16, class_path: String, method_name: String, method_signature: String) -> Frame {
        let mut locals = Vec::with_capacity(max_locals as usize);
        for _ in 0..max_locals {
            locals.push(Primitive::Null)
        }
        
        Frame {
            class_path,
            method_name,
            method_signature,

            locals,
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

    pub fn stack_peek_reverse(&mut self, n: usize) -> &Primitive {
        &self.stack[self.stack.len()-1-n]
    }

//    pub fn stack_pop_byte(&mut self) -> u8 {
//        match self.stack_pop() {
//            Primitive::Byte(v) => v,
//            p => panic!("Expected to pop Byte from stack but found: {:?}", p),
//        }
//    }

    pub fn stack_pop_boolean(&mut self) -> bool {
        match self.stack_pop() {
            Primitive::Boolean(v) => v,
            Primitive::Int(v) => v == 1,
            p => panic!("Expected to pop Boolean from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_int(&mut self) -> i32 {
        match self.stack_pop() {
            Primitive::Int(v) => v,
            Primitive::Boolean(v) => if v { 1 } else { 0 },
            Primitive::Char(v) => v as i32,
            Primitive::Byte(v) => v as i32,
            p => panic!("Expected to pop Int from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_long(&mut self) -> i64 {
        match self.stack_pop() {
            Primitive::Long(v) => v,
            p => panic!("Expected to pop Long from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_float(&mut self) -> f32 {
        match self.stack_pop() {
            Primitive::Float(v) => v,
            p => panic!("Expected to pop Float from stack but found: {:?}", p),
        }
    }

    pub fn stack_pop_double(&mut self) -> f64 {
        match self.stack_pop() {
            Primitive::Double(v) => v,
            p => panic!("Expected to pop Double from stack but found: {:?}", p),
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

//    pub fn locals_get(&mut self, index: usize) -> &Primitive {
//        self.locals.get(index).unwrap()
//    }

    pub fn locals_get_int(&mut self, index: usize) -> i32 {
        match self.locals.get(index).unwrap() {
            &Primitive::Int(ref value) => value.clone(),
            p => panic!("Expected to get Int from locals but found: {:?}", p),
        }
    }

    pub fn locals_get_float(&mut self, index: usize) -> f32 {
        match self.locals.get(index).unwrap() {
            &Primitive::Float(ref value) => value.clone(),
            p => panic!("Expected to get Float from locals but found: {:?}", p),
        }
    }

    pub fn locals_get_long(&mut self, index: usize) -> i64 {
        match self.locals.get(index).unwrap() {
            &Primitive::Long(ref value) => value.clone(),
            p => panic!("Expected to get Long from locals but found: {:?}", p),
        }
    }

    pub fn locals_get_returnaddress(&mut self, index: usize) -> u16 {
        match self.locals.get(index).unwrap() {
            &Primitive::ReturnAddress(ref address) => address.clone(),
            p => panic!("Expected to get ReturnAddress from locals but found: {:?}", p),
        }
    }

    pub fn locals_get_reference(&mut self, index: usize) -> &Primitive {
        let value = self.locals.get(index).unwrap();
        match value {
            &Primitive::Arrayref(_) => (),
            &Primitive::Objectref(_) => (),
            &Primitive::Null => (),
            _ => panic!("Expected to get reference from locals but found: {:?}", value),
        };

        value
    }

}

impl fmt::Display for Frame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Method name
        fmt.write_str("Frame of ")?;
        fmt.write_str(&self.class_path)?;
        fmt.write_str(".")?;
        fmt.write_str(&self.method_name)?;
        fmt.write_str(&self.method_signature)?;
        fmt.write_str("\nStack:\n")?;

/*        for primitive in self.stack.iter() {
            let desc = format!("  - {}\n", primitive);
            fmt.write_str(&desc)?;
        }
*/
        Ok(())
    }
}
