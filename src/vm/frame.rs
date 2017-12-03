#[derive(Debug)]
pub struct Frame {

    parent: Option<Box<Frame>>,
    locals: Vec<u32>,
    stack: Vec<u32>,

}

impl Frame {

    pub fn new(parent: Option<Box<Frame>>) -> Frame {
        Frame {
            parent,
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

    pub fn stack_push_int(&mut self, val: i32) {
        self.stack.push(val as u32);
    }

    pub fn stack_push_long(&mut self, val: i64) {
        let (high, low) = i64_to_high_low(val);
        self.stack.push(high);
        self.stack.push(low);
    }

    pub fn stack_pop_int(&mut self) -> i32 {
        self.stack.pop().unwrap() as i32
    }

    pub fn stack_pop_long(&mut self) -> i64 {
        let low = self.stack.pop().unwrap();
        let high = self.stack.pop().unwrap();

        high_low_to_i64(high, low)
    }

}

fn i64_to_high_low(val: i64) -> (u32, u32) {
    let high = (val >> 32) as u32;
    let low = val as u32;

    (high, low)
}

fn high_low_to_i64(high: u32, low: u32) -> (i64) {
    let intermediate: u64 = (high as u64) << 32;

    (intermediate + low as u64) as i64
}
