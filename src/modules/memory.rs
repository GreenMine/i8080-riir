use std::{fmt, ops};

pub struct Memory {
    pub mem: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            mem: vec![0u8; size],
        }
    }

    pub fn get_memory(&self, start: usize, amount: usize) -> &[u8] {
        &self.mem[start..amount]
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:x?}", self.get_memory(0x0100, 0x0100 + 0xF))
    }
}

impl ops::Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.mem[index as usize]
    }
}

impl ops::Index<std::ops::Range<u16>> for Memory {
    type Output = [u8];
    fn index(&self, range: std::ops::Range<u16>) -> &Self::Output {
        &self.mem[range.start as usize..range.end as usize]
    }
}

impl ops::IndexMut<std::ops::Range<u16>> for Memory {
    fn index_mut(&mut self, range: std::ops::Range<u16>) -> &mut Self::Output {
        &mut self.mem[range.start as usize..range.end as usize]
    }
}
