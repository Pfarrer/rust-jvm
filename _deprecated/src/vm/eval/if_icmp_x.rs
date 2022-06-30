use vm::utils;
use vm::Vm;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
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
        let branchoffset = utils::read_i16_code(code, pc);
        let target_pc: u16 = (pc as i16 + branchoffset) as u16;

        Some(target_pc)
    } else {
        Some(pc + 3)
    }
}
