use crate::VmThread;

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = *code.get((pc + 1) as usize).unwrap() as u8;

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let return_address = frame.locals_get_returnaddress(index as usize);

    trace!(
        "ret: Read index={} from code and return address={} from locals, branch to there..=",
        index,
        return_address
    );

    Some(return_address as u16)
}
