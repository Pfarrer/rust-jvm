use vm::primitive::Primitive;
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

    pub fn locals_get_long(&mut self, index: usize) -> i64 {
        match self.locals.get(index).unwrap() {
            &Primitive::Long(ref value) => value.clone(),
            p => panic!("Expected to get Long from locals but found: {:?}", p),
        }
    }

    pub fn locals_get_reference(&mut self, index: usize) -> &Box<Instance> {
        match self.locals.get(index).unwrap() {
            &Primitive::Reference(ref boxed) => boxed,
            p => panic!("Expected to get Reference from locals but found: {:?}", p),
        }
    }
}