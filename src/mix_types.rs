// Per the specification:
// "Each byte holds an unspecified amount of information, but
// must contain at least 64 distinct values and at most 100 distinct values."
//
// This implementation uses exactly 64 values (6 bits).
// The lower 6 bits store the MIX byte data. The top two bits are unused.
pub type MixByte = u8;

// "A computer word consists of five bytes and a sign."
//
// The bytes are packed in this representation. Six bits are used for each
// byte, in order. Layout is:
// .            .      ......  ......  ......  ......  ......
// [unused bit] [sign] [byte1] [byte2] [byte3] [byte4] [byte5]
// The sign bit is 1 for -, 0 for +
pub type MixWord = u32;

// Convenience type for two bytes and a sign. This the size of all the index
// registers, the jump register (almost), and any computed address.
// Also packed. Layout is:
// ...           .      ......  ......
// [unused bits] [sign] [byte1] [byte2]
pub type MixAddr = u16;

// "There are nine registers in MIX"
pub struct MixRegisters {
    pub a: MixWord,
    pub x: MixWord,
    pub i: [MixAddr; 6],
    // Note that the J register always behaves as though its sign is +
    pub j: MixAddr,
}

pub type MixMemory = [MixWord; 4000];

pub enum MixCompare {
    Less,
    Equal,
    Greater,
}

pub struct MixState<'a> {
    pub r: &'a MixRegisters,
    pub comparison: MixCompare,
    pub overflow: bool,
    pub memory: &'a MixMemory,
    pub io: (), // TODO
    // Inexplicably, a program counter is not documented explicitly. Presumably
    // this is because it's an implementation detail. Still, we need it!
    pub pc: u32,
}

