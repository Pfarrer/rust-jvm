use classfile::Classfile;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop_int();

    let cmp_result = match *code.get(pc as usize).unwrap() {
        153 => value == 0, // ifeq
        154 => value != 0, // ifne
        155 => value < 0, // iflt
        156 => value <= 0, // ifle
        157 => value > 0, // ifgt
        158 => value >= 0, // ifge
        _ => panic!("ifX::eval was called on a non ifX instruction."),
    };

    trace!("ifX ({}): {} -> {}", code.get(pc as usize).unwrap(), value, cmp_result);

    if cmp_result {
        let branchbyte1: u16 = (*code.get((pc + 1) as usize).unwrap() as u16) << 8;
        let branchbyte2: u16 = (*code.get((pc + 2) as usize).unwrap()) as u16;
        let branchoffset = branchbyte1 + branchbyte2;

        Some(pc + branchoffset)
    } else {
        Some(pc + 3)
    }
}