use crate::assembler::unsigned_newtypes::U6;

/// A simpler nibble-packing function that arranges 8 nibbles into a 32-bit word.
/// nibble[0] goes to the top 4 bits, nibble[7] goes to the bottom 4 bits:
pub fn pack_nibbles(nibbles: [u32; 8]) -> u32 {
    let mut out = 0;
    for (i, &nib) in nibbles.iter().enumerate() {
        // nib 0 goes to bits [31..28],
        // nib 1 goes to bits [27..24], etc.
        out |= nib << (4 * (7 - i));
    }
    out
}

/// Split a u16 into four nibbles (high nibble first).
pub fn split_u16_into_nibbles(v: u16) -> (u32, u32, u32, u32) {
    let v = v as u32;
    (
        (v >> 12) & 0xF,
        (v >>  8) & 0xF,
        (v >>  4) & 0xF,
        (v >>  0) & 0xF,
    )
}

/// Split a U6 into two nibbles.
pub fn split_u6_into_nibbles(v: U6) -> (u32, u32) {
    let val = v.get() as u32;
    (
        (val >> 4) & 0xF,
        (val >> 0) & 0xF,
    )
}

pub fn denibble(nibbles: [u32; 8]) -> u32 {
    const MASKS: [u32; 8] = [
        0xF0000000,
        0x0F000000,
        0x00F00000,
        0x000F0000,
        0x0000F000,
        0x00000F00,
        0x000000F0,
        0x0000000F,
    ];

    let mut out = 0;

    nibbles
        .iter()
        .enumerate()
        .map(|(i, n)| (n << MASKS[i].trailing_zeros()) & MASKS[i])
        .for_each(|n| out |= n);

    out
}

pub fn nibbles_u16(v: u16) -> (u32, u32, u32, u32) {
    const MASKS: [u32; 4] = [
        0xF000,
        0x0F00,
        0x00F0,
        0x000F,
    ];

    let v = v as u32;

    (
        (v & MASKS[0]) >> MASKS[0].trailing_zeros(),
        (v & MASKS[1]) >> MASKS[1].trailing_zeros(),
        (v & MASKS[2]) >> MASKS[2].trailing_zeros(),
        (v & MASKS[3]) >> MASKS[3].trailing_zeros(),
    )
}

pub fn nibbles_u6(v: U6) -> (u32, u32) {
    const MASKS: [u32; 2] = [
        0xF0,
        0x0F,
    ];

    let v = v.get() as u32;

    (
        (v & MASKS[0]) >> MASKS[0].trailing_zeros(),
        (v & MASKS[1]) >> MASKS[1].trailing_zeros(),
    )
}
