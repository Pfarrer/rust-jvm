use vm::utils;
use vm::Vm;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    let (cmp_result, instr_name) = match *code.get(pc as usize).unwrap() {
        153 => (value == 0, "ifeq"),
        154 => (value != 0, "ifne"),
        155 => (value < 0, "iflt"),
        156 => (value >= 0, "ifge"),
        157 => (value > 0, "ifgt"),
        158 => (value <= 0, "ifle"),
        i => panic!("if_x::eval was called on a non if_x instruction: {}", i),
    };

    trace!("{}: {} -> {}", instr_name, value, cmp_result);

    if cmp_result {
        let branchoffset = utils::read_u16_code(code, pc);
        Some(pc + branchoffset)
    } else {
        Some(pc + 3)
    }
}
