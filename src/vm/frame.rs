use vm::types::Primitive;

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
}