use vm::Frame;
use vm::utils;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();

    let (cmp_result, instr_name) = match *code.get(pc as usize).unwrap() {
        159 => (value1 == value2, "eq"),
        160 => (value1 != value2, "ne"),
        161 => (value1 < value2, "lt"),
        162 => (value1 >= value2, "ge"),
        163 => (value1 > value2, "gt"),
        164 => (value1 <= value2, "le"),
        _ => panic!("if_icmp_x::eval was called on a non if_icmp_x instruction."),
    };

    trace!("if_icmp{}: {} and {} -> {}", instr_name, value1, value2, cmp_result);

    if cmp_result {
        let branchoffset = utils::read_u16_code(code, pc);
        Some(pc + branchoffset)
    } else {
        Some(pc + 3)
    }
}