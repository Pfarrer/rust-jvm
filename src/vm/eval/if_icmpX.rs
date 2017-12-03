use classfile::Classfile;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();

    let cmp_result = match *code.get(pc as usize).unwrap() {
        159 => value1 == value2, // if_icmpeq
        160 => value1 != value2, // if_icmpne
        161 => value1 < value2, // if_icmplt
        162 => value1 <= value2, // if_icmpge
        163 => value1 > value2, // if_icmpgt
        164 => value1 >= value2, // if_icmple
        _ => panic!("if_icmpX::eval was called on a non if_icmpX instruction."),
    };

    trace!("if_icmpX ({}): {} and {} -> {}", code.get(pc as usize).unwrap(), value1, value2, cmp_result);

    if cmp_result {
        let branchbyte1: u16 = (*code.get((pc + 1) as usize).unwrap() as u16) << 8;
        let branchbyte2: u16 = (*code.get((pc + 2) as usize).unwrap()) as u16;
        let branchoffset = branchbyte1 + branchbyte2;

        Some(pc + branchoffset)
    } else {
        Some(pc + 3)
    }
}