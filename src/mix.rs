// Per the specification:
// "Each byte holds an unspecified amount of information, but
// must contain at least 64 distinct values and at most 100 distinct values."
//
// This implementation uses exactly 64 values (6 bits).
// The lower 6 bits store the MIX byte data. The top two bits are unused.
type MixByte = u8;

// "A computer word consists of five bytes and a sign."
//
// The bytes are packed in this representation. Six bits are used for each
// byte, in order. Layout is:
// .            .      ......  ......  ......  ......  ......
// [unused bit] [sign] [byte1] [byte2] [byte3] [byte4] [byte5]
// The sign bit is 1 for -, 0 for +
type MixWord = u32;

// Convenience type for two bytes and a sign. This the size of all the index
// registers, the jump register (almost), and any computed address.
// Also packed. Layout is:
// ...           .      ......  ......
// [unused bits] [sign] [byte1] [byte2]
type MixAddr = u16;

// "There are nine registers in MIX"
struct MixRegisters {
    a: MixWord,
    x: MixWord,
    i: [MixAddr; 6],
    // Note that the J register always behaves as though its sign is +
    j: MixAddr,
}

type MixMemory = [MixWord; 4000];

enum MixCompare {
    Less,
    Equal,
    Greater,
}

struct MixState<'a> {
    r: &'a MixRegisters,
    comparison: MixCompare,
    overflow: bool,
    memory: &'a MixMemory,
    io: (), // TODO
    // Inexplicably, a program counter is not documented explicitly. Presumably
    // this is because it's an implementation detail. Still, we need it!
    pc: u32,
}

// Helpful constants
const ONES: u32 = !0;

// Unlike MIX, my computer doesn't have separate sign bits.
// So we need these utility functions anytime we want to do arithmetic.

// Convert a MIX address to a signed machine 16-bit integer.
fn from_mix_addr(addr: MixAddr) -> i16 {
    let iaddr = (addr & ((!0) >> 4)) as i16;
    let sign = ((addr >> 12) & 1) as i16;
    return (sign * 2 - 1) * iaddr;
}

// Convert a signed machine 16-bit integer to a MIX address.
// This function assumes the value is small enough to fit in a MIX
// address (any larger bits are truncated).
fn to_mix_addr(addr: i16) -> MixAddr {
    let uaddr;
    if addr < 0 {
        uaddr = (-addr) as u16;
    }
    else {
        uaddr = addr as u16;
    }
    let sign_bit = (addr < 0) as u16;
    return (sign_bit << 12) | (uaddr & ((!0) >> 4));
}

// Convert a MIX word to a signed machine 32-bit integer.
fn from_mix_word(word: MixWord) -> i32 {
    let iword = (word & ((!0) >> 2)) as i32;
    let sign = ((word >> 30) & 1) as i32;
    return (sign * 2 - 1) * iword;
}

// Convert a signed machine 32-bit integer to a MIX word.
// This function assumes the value is small enough to fit in a MIX
// word (any larger bits are truncated).
fn to_mix_word(word: i32) -> MixWord {
    let uword;
    if word < 0 {
        uword = (-word) as u32;
    }
    else {
        uword = word as u32;
    }
    let sign_bit = (word < 0) as u32;
    return (sign_bit << 30) | (uword & ((!0) >> 2));
}

// Utility function.
// Returns bytes from the given byte up to and including the given byte,
// from within a mix word. From and to must be between 0 and 5, inclusive.
// The result is returned in the lowest-order bits of the u32 return argument.
// Note that if the sign is requested its bit will be shifted over as well!
fn get_bytes(word: MixWord, from: u8, to: u8) -> u32 {
    if from == 0 {
        return word >> (6 * (5 - to));
    } else {
        return (word >> (6 * (5 - to))) & (ONES >> (32 - 6 * (to - from + 1)));
    }
}

// Apply the given field specification to the given word. This zeroes out
// all parts of the word not included in the field specification. All parts
// of the word within the field specification are retained.
fn get_field_word(word: MixWord, field_spec: MixByte) -> MixWord {
    let l = field_spec / 8;
    let r = field_spec % 8;
    return get_bytes(word, l, r) << (6 * (5 - r));
}

fn effective_address(instr: MixWord, r: &MixRegisters) -> MixAddr {
    let base_addr = get_bytes(instr, 0, 2) as u16;
    let i = get_bytes(instr, 3, 3) as usize;
    if i == 0 {
        return base_addr;
    }
    else {
        return to_mix_addr(from_mix_addr(base_addr) + from_mix_addr(r.i[i-1]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_to_from_mix_word_addr() {
        assert_eq!(from_mix_word(0), 0);
        assert_eq!(from_mix_word(1), 1);
        assert_eq!(from_mix_word((1 << 30) | 1), -1);
        assert_eq!(from_mix_word((1 << 30) | 3), -3);

        assert_eq!(to_mix_word(0), 0);
        assert_eq!(to_mix_word(1), 1);
        assert_eq!(to_mix_word(-1), (1 << 30) | 1);
        assert_eq!(to_mix_word(-3), (1 << 30) | 3);

        assert_eq!(from_mix_addr(0), 0);
        assert_eq!(from_mix_addr(1), 1);
        assert_eq!(from_mix_addr((1 << 30) | 1), -1);
        assert_eq!(from_mix_addr((1 << 30) | 3), -3);

        assert_eq!(to_mix_addr(0), 0);
        assert_eq!(to_mix_addr(1), 1);
        assert_eq!(to_mix_addr(-1), (1 << 30) | 1);
        assert_eq!(to_mix_addr(-3), (1 << 30) | 3);
    }

    #[test]
    fn test_get_bytes_field() {
        let test0 = 0;
        let test1 = 0b00_100000_010101_000000_000000_000000;
        let test2 = 0b01_100000_010101_100000_000001_000000;

        assert_eq!(get_bytes(test0, 0, 0), 0);
        assert_eq!(get_bytes(test0, 1, 3), 0);
        assert_eq!(get_bytes(test0, 1, 5), 0);
        assert_eq!(get_bytes(test0, 0, 5), 0);
        assert_eq!(get_bytes(test1, 0, 5), test1);
        assert_eq!(get_bytes(test1, 0, 0), 0);
        assert_eq!(get_bytes(test1, 0, 1), 0b100000);
        assert_eq!(get_bytes(test1, 0, 2), 0b100000_010101);
        assert_eq!(get_bytes(test1, 1, 2), 0b100000_010101);
        assert_eq!(get_bytes(test1, 1, 3), 0b100000_010101_000000);
        assert_eq!(get_bytes(test1, 1, 5), test1);
        assert_eq!(get_bytes(test2, 0, 1), 0b01_100000);
        assert_eq!(get_bytes(test2, 0, 2), 0b01_100000_010101);
        assert_eq!(get_bytes(test2, 1, 2), get_bytes(test1, 1, 2));

        assert_eq!(get_field_word(0, 0), 0);
        assert_eq!(get_field_word(0, 11), 0);
        assert_eq!(get_field_word(0, 13), 0);
        assert_eq!(get_field_word(0, 5), 0);
        assert_eq!(get_field_word(test1, 5), test1);
        assert_eq!(get_field_word(test1, 0), 0);
        assert_eq!(get_field_word(test1, 1),
            0b00_100000_000000_000000_000000_000000);
        assert_eq!(get_field_word(test1, 2),
            0b00_100000_010101_000000_000000_000000);
        assert_eq!(get_field_word(test1, 10),
            0b00_100000_010101_000000_000000_000000);
        assert_eq!(get_field_word(test1, 11),
            0b00_100000_010101_000000_000000_000000);
        assert_eq!(get_field_word(test1, 13), test1);
        assert_eq!(get_field_word(test2, 1),
            0b01_100000_000000_000000_000000_000000);
        assert_eq!(get_field_word(test2, 2),
            0b01_100000_010101_000000_000000_000000);
        assert_eq!(get_field_word(test2, 10), get_field_word(test1, 10));

    }
}
