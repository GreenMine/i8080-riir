use std::{fmt, ops};

pub struct Ram<T>(Vec<T>);

pub struct Memory {
    pub(crate) stack: Ram<u16>,
    pub(crate) program: Ram<u8>,
    pub(crate) heap: Ram<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            stack: Ram(vec![0u16; 0x80]),
            program: Ram(vec![0u8; 0x400]),
            heap: Ram(vec![0u8; 0xFB00]),
        }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Stack: {:x?}\nHeap: {:x?}",
            &self.stack[0x0..0x1F],
            &self.heap[0x0..0x1F]
        )
    }
}

impl<T> ops::Index<u16> for Ram<T> {
    type Output = T;
    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl<T> ops::IndexMut<u16> for Ram<T> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T> ops::Index<ops::Range<u16>> for Ram<T> {
    type Output = [T];
    fn index(&self, range: std::ops::Range<u16>) -> &Self::Output {
        &self.0[range.start as usize..range.end as usize]
    }
}
impl<T> ops::IndexMut<ops::Range<u16>> for Ram<T> {
    fn index_mut(&mut self, range: std::ops::Range<u16>) -> &mut Self::Output {
        &mut self.0[range.start as usize..range.end as usize]
    }
}
