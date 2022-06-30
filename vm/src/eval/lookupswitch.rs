use vm::utils;
use vm::Vm;

pub fn eval(vm: &Vm, pc: u16, code: &Vec<u8>) -> Option<u16> {
    // The following is 4-byte aligned, thus, there can be up to 3 padding bytes
    let pad_bytes = get_number_of_pad_bytes(pc);
    let default = utils::read_i32_code(code, pc, pad_bytes);
    let npairs = utils::read_i32_code(code, pc, pad_bytes + 4);
    let frame = vm.frame_stack.last_mut().unwrap();
    let key = frame.stack_pop_int();

    trace!("Switch found with key={} and {} cases", key, npairs);
    for i in 0..npairs {
        // Read match-offset pair
        let match_ = utils::read_i32_code(code, pc, pad_bytes + 8 + 4 * 2 * i as u16);
        let offset = utils::read_i32_code(code, pc, pad_bytes + 8 + 4 * 2 * i as u16 + 4);
        if match_ == key {
            // Yeah, match found -> branch using offset
            trace!("Match found, branching now..+");
            return Some((pc as i32 + offset) as u16);
        }
    }

    trace!("No match found, branching to default block..+");
    Some((pc as i32 + default) as u16)
}

fn get_number_of_pad_bytes(pc: u16) -> u16 {
    if pc % 4 == 0 {
        0
    } else {
        4 - (pc % 4) as u16
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_number_of_pad_bytes() {
        assert_eq!(1, super::get_number_of_pad_bytes(39));
        assert_eq!(0, super::get_number_of_pad_bytes(40));
        assert_eq!(3, super::get_number_of_pad_bytes(41));
        assert_eq!(2, super::get_number_of_pad_bytes(42));
    }
}
