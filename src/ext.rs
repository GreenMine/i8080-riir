pub fn split_slice(slice: &[u8]) -> u16 {
    (slice[1] as u16) << 8 | slice[0] as u16
}
