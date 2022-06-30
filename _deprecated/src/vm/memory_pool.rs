use std::mem::transmute;

pub struct MemoryPool {
    pool: Option<Vec<u8>>,
}

impl MemoryPool {

    pub fn new() -> MemoryPool {
        MemoryPool {
            pool: None,
        }
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        match self.pool {
            None => {
                info!("Allocating {} bytes of memory", size);
                self.pool = Some(Vec::with_capacity(size));
            },
            _ => panic!("A memory pool was allocated previously, this implementation can only handle one pool!"),
        }

        0
    }

    pub fn free(&mut self) {
        match self.pool {
            None => panic!("No active allocation!"),
            Some(_) => {
                trace!("Memory freed!");
                self.pool = None;
            },
        }
    }

    pub unsafe fn put_long(&mut self, address: usize, value: i64) {
        let mut p = self.get_mut_ptr().add(address);

        let bytes: [u8; 8] = transmute(value);
        for b in bytes.iter() {
            p.write(*b);
            p = p.add(1);
        }
    }

    pub unsafe fn get_byte(&mut self, address: usize) -> u8 {
        let p = self.get_ptr().add(address);
        
        trace!("Read byte {} from raw memory at address {}", *p, address);

        *p
    }

    unsafe fn get_ptr(&self) -> *const u8 {
        match self.pool {
            None => panic!("No memory was allocated previously"),
            Some(ref vec) => vec.as_ptr(),
        }
    }

    unsafe fn get_mut_ptr(&mut self) -> *mut u8 {
        match self.pool {
            None => panic!("No memory was allocated previously"),
            Some(ref mut vec) => vec.as_mut_ptr(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_usage() {
        let mut memory_pool = MemoryPool::new();
        
        unsafe {
            let p = memory_pool.allocate(8);
            memory_pool.put_long(p, 0x0102030405060708);

            assert_eq!(memory_pool.get_byte(p), 0x08);
            assert_eq!(memory_pool.get_byte(p+2), 0x06);
            assert_eq!(memory_pool.get_byte(p+4), 0x04);
            assert_eq!(memory_pool.get_byte(p+7), 0x01);
        }
    }

    #[test]
    fn test_like_in_jvm_init() {
        let mut memory_pool = MemoryPool::new();
        
        unsafe {
            let p = memory_pool.allocate(8);
            memory_pool.put_long(p, 72623859790382856);

            assert_eq!(memory_pool.get_byte(p), 0x08);
        }
    }
}