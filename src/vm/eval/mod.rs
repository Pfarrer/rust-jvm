mod getstatic;
mod putstatic;
mod invokevirtual;
mod invokespecial;
mod invokestatic;
mod new;
mod lconst;
mod lcmp;
mod if_x;
mod if_icmp_x;
mod aconst_null;
mod areturn;
mod return_;
mod ldc_x;
mod dup;
mod aload_x;

use classfile::Classfile;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame, parent_frame: &mut Frame) -> Option<u16> {
    match *code.get(pc as usize).unwrap() {
        1 => aconst_null::eval(pc, frame),
        9 => lconst::eval(0, pc, frame),
        10 => lconst::eval(1, pc, frame),
        18 => ldc_x::eval(vm, class, code, pc, frame),
        19 => ldc_x::eval(vm, class, code, pc, frame),
        42...45 => aload_x::eval(code, pc, frame),
        89 => dup::eval(pc, frame),
        148 => lcmp::eval(pc, frame),
        153...158 => if_x::eval(code, pc, frame),
        159...164 => if_icmp_x::eval(code, pc, frame),
        176 => areturn::eval(frame, parent_frame),
        177 => return_::eval(),
        178 => getstatic::eval(vm, class, code, pc, frame),
        179 => putstatic::eval(vm, class, code, pc, frame),
        182 => invokevirtual::eval(vm, class, code, pc, frame),
        183 => invokespecial::eval(vm, class, code, pc, frame),
        184 => invokestatic::eval(vm, class, code, pc, frame),
        187 => new::eval(vm, class, code, pc, frame),
        instr => panic!("Instruction not implemented: {}", instr),
    }
}