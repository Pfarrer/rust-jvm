mod getstatic;

use classfile::Classfile;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    match *code.get(pc as usize).unwrap() {
        178 => getstatic::eval(vm, class, code, pc, frame),
        instr => panic!("Instruction not implemented: {}", instr),
    }
}