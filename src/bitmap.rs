use core::ops::{Index};

struct BitMap {
    bitmap: &'static mut [u8],
    size: usize,
}

impl BitMap {
    pub fn get(&self, i: usize) -> Result<bool, ()> {
        if i > (self.size * 8) {
            return Err(())
        }
        let byte_index = i / 8;
        let bit_index = i & 8;
        let bit_indexer = 0b10000000 >> bit_index;
        if (self.bitmap[byte_index] & bit_indexer) > 0 {
            return Ok(true)
        }
        Ok(false)
    }
    pub fn set(&mut self, i: usize, value: bool) -> Result<bool, ()> {
        if i > (self.size * 8) {
            return Err(())
        }

        let byte_index = i / 8;
        let bit_index = i & 8;
        let bit_indexer = 0b10000000 >> bit_index;
        self.bitmap[byte_index] &= !bit_indexer;
        if value {
            self.bitmap[byte_index] |= bit_indexer;
        }
        return Ok(value)
    }
}

impl Index<usize> for BitMap {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index).unwrap()
    }
}