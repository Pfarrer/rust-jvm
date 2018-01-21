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
mod astore_x;
mod ldc2_w;
mod lstore_x;
mod lload_x;
mod goto;
mod ladd;
mod bipush;
mod newarray;
mod iconst_x;
mod castore;
mod istore_x;
mod iload_x;
mod arraylength;
mod iinc;
mod isub;
mod anewarray;
mod getfield;
mod putfield;
mod monitorenter;
mod ireturn;
mod lreturn;
mod pop;
mod pop2;
mod i2l;
mod iadd;
mod ifnonnull;
mod sipush;
mod checkcast;
mod iastore;
mod fload_x;
mod fconst_x;
mod fcmp_x;
mod dup2_x1;
mod caload;

use classfile::Classfile;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame, parent_frame: &mut Frame) -> Option<u16> {
    trace!("{}.{}{}#{}", frame.class_path, frame.method_name, frame.method_signature, pc);

    match *code.get(pc as usize).unwrap() {
        0 => Some(pc + 1),
        1 => aconst_null::eval(pc, frame),
        2...8 => iconst_x::eval(code, pc, frame),
        9 => lconst::eval(0, pc, frame),
        10 => lconst::eval(1, pc, frame),
        11...13 => fconst_x::eval(code, pc, frame),
        16 => bipush::eval(code, pc, frame),
        17 => sipush::eval(code, pc, frame),
        18 => ldc_x::eval(vm, class, code, pc, frame),
        19 => ldc_x::eval(vm, class, code, pc, frame),
        20 => ldc2_w::eval(class, code, pc, frame),
        21 => iload_x::eval(code, pc, frame),
        22 => lload_x::eval(code, pc, frame),
        23 => fload_x::eval(code, pc, frame),
        26...29 => iload_x::eval(code, pc, frame),
        30...33 => lload_x::eval(code, pc, frame),
        34...37 => fload_x::eval(code, pc, frame),
        42...45 => aload_x::eval(code, pc, frame),
        52 => caload::eval(pc, frame),
        54 => istore_x::eval(code, pc, frame),
        55 => lstore_x::eval(code, pc, frame),
        58 => astore_x::eval(code, pc, frame),
        59...62 => istore_x::eval(code, pc, frame),
        63...66 => lstore_x::eval(code, pc, frame),
        75...78 => astore_x::eval(code, pc, frame),
        79 => iastore::eval(pc, frame),
        85 => castore::eval(pc, frame),
        87 => pop::eval(pc, frame),
        88 => pop2::eval(pc, frame),
        89 => dup::eval(pc, frame),
        93 => dup2_x1::eval(pc, frame),
        96 => iadd::eval(pc, frame),
        97 => ladd::eval(pc, frame),
        100 => isub::eval(pc, frame),
        132 => iinc::eval(code, pc, frame),
        133 => i2l::eval(pc, frame),
        148 => lcmp::eval(pc, frame),
        149...150 => fcmp_x::eval(code, pc, frame),
        153...158 => if_x::eval(code, pc, frame),
        159...164 => if_icmp_x::eval(code, pc, frame),
        167 => goto::eval(code, pc),
        172 => ireturn::eval(frame, parent_frame),
        173 => lreturn::eval(frame, parent_frame),
        176 => areturn::eval(frame, parent_frame),
        177 => return_::eval(),
        178 => getstatic::eval(vm, class, code, pc, frame),
        179 => putstatic::eval(vm, class, code, pc, frame),
        180 => getfield::eval(class, code, pc, frame),
        181 => putfield::eval(class, code, pc, frame),
        182 => invokevirtual::eval(vm, class, code, pc, frame),
        183 => invokespecial::eval(vm, class, code, pc, frame),
        184 => invokestatic::eval(vm, class, code, pc, frame),
        187 => new::eval(vm, class, code, pc, frame),
        188 => newarray::eval(code, pc, frame),
        189 => anewarray::eval(class, code, pc, frame),
        190 => arraylength::eval(pc, frame),
        192 => checkcast::eval(pc),
        194 => monitorenter::eval(pc, frame),
        199 => ifnonnull::eval(code, pc, frame),
        instr => panic!("Instruction not implemented: {}", instr),
    }
}