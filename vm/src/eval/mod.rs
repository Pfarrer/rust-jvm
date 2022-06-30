use log::trace;

// mod aaload;
// mod aastore;
// mod aconst_null;
// mod aload_x;
// mod anewarray;
// mod areturn;
// mod arraylength;
// mod astore_x;
// mod baload;
// mod bastore;
// mod bipush;
// mod caload;
// mod castore;
// mod checkcast;
// mod dup;
// mod dup2;
// mod dup2_x1;
// mod dup_x1;
// mod f2i;
// mod fcmp_x;
// mod fconst_x;
// mod fload_x;
// mod fmul;
// mod getfield;
// mod getstatic;
// mod goto;
// mod i2b;
// mod i2c;
// mod i2f;
// mod i2l;
// mod iadd;
// mod iand;
// mod iastore;
// mod iconst_x;
// mod if_acmp_x;
// mod if_icmp_x;
// mod if_x;
// mod ifnonnull;
// mod ifnull;
// mod iinc;
// mod iload_x;
// mod imul;
// mod instanceof;
// mod invokeinterface;
// mod invokespecial;
// mod invokestatic;
// mod invokevirtual;
// mod ior;
// mod irem;
// mod ireturn;
// mod ishl;
// mod ishr;
// mod istore_x;
// mod isub;
// mod iushr;
// mod ixor;
// mod jsr;
// mod ladd;
// mod land;
// mod lcmp;
// mod lconst;
// mod ldc2_w;
// mod ldc_x;
// mod lload_x;
// mod lookupswitch;
// mod lreturn;
// mod lstore_x;
// mod lxor;
// mod monitorenter;
// mod monitorexit;
// mod new;
// mod newarray;
// mod pop;
// mod pop2;
// mod putfield;
// mod putstatic;
// mod ret;
// mod return_;
// mod sipush;

use crate::vm_thread::VmThread;
use model::class::JvmClass;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let instr = *code.get(pc as usize).unwrap();

    {
        let frame = vm_thread.frame_stack.last().unwrap();
        trace!(
            "{}.{}{}#{} = {}",
            frame.class_path,
            frame.method_name,
            frame.method_signature,
            pc,
            instr
        );
    }

    match instr {
        0 => Some(pc + 1),
        // 1 => aconst_null::eval(vm, pc),
        // 2..+8 => iconst_x::eval(vm, code, pc),
        // 9 => lconst::eval(0, vm, pc),
        // 10 => lconst::eval(1, vm, pc),
        // 11..+13 => fconst_x::eval(vm, code, pc),
        // 16 => bipush::eval(vm, code, pc),
        // 17 => sipush::eval(vm, code, pc),
        // 18 => ldc_x::eval(vm, class, code, pc),
        // 19 => ldc_x::eval(vm, class, code, pc),
        // 20 => ldc2_w::eval(vm, class, code, pc),
        // 21 => iload_x::eval(vm, code, pc),
        // 22 => lload_x::eval(vm, code, pc),
        // 23 => fload_x::eval(vm, code, pc),
        // 25 => aload_x::eval(vm, code, pc),
        // 26..+29 => iload_x::eval(vm, code, pc),
        // 30..+33 => lload_x::eval(vm, code, pc),
        // 34..+37 => fload_x::eval(vm, code, pc),
        // 42..+45 => aload_x::eval(vm, code, pc),
        // 50 => aaload::eval(vm, pc),
        // 51 => baload::eval(vm, pc),
        // 52 => caload::eval(vm, pc),
        // 54 => istore_x::eval(vm, code, pc),
        // 55 => lstore_x::eval(vm, code, pc),
        // 58 => astore_x::eval(vm, code, pc),
        // 59..+62 => istore_x::eval(vm, code, pc),
        // 63..+66 => lstore_x::eval(vm, code, pc),
        // 75..+78 => astore_x::eval(vm, code, pc),
        // 79 => iastore::eval(vm, pc),
        // 83 => aastore::eval(vm, pc),
        // 84 => bastore::eval(vm, pc),
        // 85 => castore::eval(vm, pc),
        // 87 => pop::eval(vm, pc),
        // 88 => pop2::eval(vm, pc),
        // 89 => dup::eval(vm, pc),
        // 90 => dup_x1::eval(vm, pc),
        // 92 => dup2::eval(vm, pc),
        // 93 => dup2_x1::eval(vm, pc),
        // 96 => iadd::eval(vm, pc),
        // 97 => ladd::eval(vm, pc),
        // 100 => isub::eval(vm, pc),
        // 104 => imul::eval(vm, pc),
        // 106 => fmul::eval(vm, pc),
        // 112 => irem::eval(vm, pc),
        // 120 => ishl::eval(vm, pc),
        // 122 => ishr::eval(vm, pc),
        // 124 => iushr::eval(vm, pc),
        // 126 => iand::eval(vm, pc),
        // 127 => land::eval(vm, pc),
        // 128 => ior::eval(vm, pc),
        // 130 => ixor::eval(vm, pc),
        // 131 => lxor::eval(vm, pc),
        // 132 => iinc::eval(vm, code, pc),
        // 133 => i2l::eval(vm, pc),
        // 134 => i2f::eval(vm, pc),
        // 139 => f2i::eval(vm, pc),
        // 145 => i2b::eval(vm, pc),
        // 146 => i2c::eval(vm, pc),
        // 148 => lcmp::eval(vm, pc),
        // 149..+150 => fcmp_x::eval(vm, code, pc),
        // 153..+158 => if_x::eval(vm, code, pc),
        // 159..+164 => if_icmp_x::eval(vm, code, pc),
        // 156..+166 => if_acmp_x::eval(vm, code, pc),
        // 167 => goto::eval(code, pc),
        // 168 => jsr::eval(vm, code, pc),
        // 169 => ret::eval(vm, code, pc),
        // 171 => lookupswitch::eval(vm, pc, code),
        // 172 => ireturn::eval(vm),
        // 173 => lreturn::eval(vm),
        // 176 => areturn::eval(vm),
        // 177 => return_::eval(),
        // 178 => getstatic::eval(vm, class, code, pc),
        // 179 => putstatic::eval(vm, class, code, pc),
        // 180 => getfield::eval(vm, class, code, pc),
        // 181 => putfield::eval(vm, class, code, pc),
        // 182 => invokevirtual::eval(vm, class, code, pc),
        // 183 => invokespecial::eval(vm, class, code, pc),
        // 184 => invokestatic::eval(vm, class, code, pc),
        // 185 => invokeinterface::eval(vm, class, code, pc),
        // 187 => new::eval(vm, class, code, pc),
        // 188 => newarray::eval(vm, code, pc),
        // 189 => anewarray::eval(vm, class, code, pc),
        // 190 => arraylength::eval(vm, pc),
        // 192 => checkcast::eval(pc),
        // 193 => instanceof::eval(vm, class, code, pc),
        // 194 => monitorenter::eval(vm, pc),
        // 195 => monitorexit::eval(vm, pc),
        // 198 => ifnull::eval(vm, code, pc),
        // 199 => ifnonnull::eval(vm, code, pc),
        instr => panic!("Instruction not implemented: {}", instr),
    }
}
