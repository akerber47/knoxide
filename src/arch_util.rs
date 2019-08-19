// Conversion functions for byte arrays.  Note that we assume a little-endian machine. That is,
// within each word/addr, the least significant byte appears first in the input array.
// For convenience, this takes in a slice, not a fixed length array,
// but it had better be the right length.

pub fn u8sto32(bytes: &[u8]) -> u32 {
    assert!(bytes.len() == 4);
    let mut w: u32 = 0;
    for b in bytes {
        w = (w >> 8) | ((*b as u32) << 24);
    }
    w
}

pub fn u8sto16(bytes: &[u8]) -> u16 {
    assert!(bytes.len() == 2);
    let mut a: u16 = 0;
    for b in bytes {
        a = (a >> 8) | ((*b as u16) << 8);
    }
    a
}

pub fn u32to8s(w: u32) -> [u8; 4] {
    let mut bytes: [u8; 4] = [0; 4];
    for i in 0 .. 4 {
        bytes[i] = (w >> (8*i)) as u8;
    }
    bytes
}

pub fn u16to8s(a: u16) -> [u8; 2] {
    let mut bytes: [u8; 2] = [0; 2];
    for i in 0 .. 2 {
        bytes[i] = (a >> (8*i)) as u8;
    }
    bytes
}


