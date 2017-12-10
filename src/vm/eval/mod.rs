mod getstatic;
mod putstatic;
mod invokestatic;
mod lconst;
mod lcmp;
mod ifX;
mod if_icmpX;
mod aconst_null;
mod areturn;
mod return_;

use classfile::Classfile;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame, parent_frame: &mut Frame) -> Option<u16> {
    match *code.get(pc as usize).unwrap() {
        1 => aconst_null::eval(pc, frame),
        9 => lconst::eval(0, pc, frame),
        10 => lconst::eval(1, pc, frame),
        148 => lcmp::eval(pc, frame),
        153...158 => ifX::eval(code, pc, frame),
        159...164 => if_icmpX::eval(code, pc, frame),
        176 => areturn::eval(frame, parent_frame),
        177 => return_::eval(),
        178 => getstatic::eval(vm, class, code, pc, frame),
        179 => putstatic::eval(vm, class, code, pc, frame),
        184 => invokestatic::eval(vm, class, code, pc, frame),
        instr => panic!("Instruction not implemented: {}", instr),
    }
}