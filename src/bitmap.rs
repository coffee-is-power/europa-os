pub struct BitMap {
    pub(crate) bitmap: *mut u8,
    pub(crate) size: usize,
}

impl BitMap {
    pub fn get(&self, i: usize) -> Result<bool, ()> {
        if i > (self.size * 8) {
            return Err(())
        }
        let byte_index = i / 8;
        let bit_index = i % 8;
        let bit_indexer = 0b10000000 >> bit_index;
        unsafe {
            if (*(self.bitmap.offset(byte_index as isize)) & bit_indexer) > 0 {
                return Ok(true)
            }
        }

        Ok(false)
    }
    pub fn set(&mut self, i: usize, value: bool) -> Result<bool, ()> {
        if i > (self.size * 8) {
            return Err(())
        }

        let byte_index = i / 8;
        let bit_index = i % 8;
        let bit_indexer = 0b10000000 >> bit_index;
        unsafe {
            *(self.bitmap.offset(byte_index as isize)) &= !bit_indexer;
        }
        if value {
            unsafe {
                *(self.bitmap.offset(byte_index as isize)) |= bit_indexer;
            }
        }
        return Ok(value)
    }
}
