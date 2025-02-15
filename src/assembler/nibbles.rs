use arbitrary_int::u6;

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
         v        & 0xF,
    )
}

/// Split a U6 into two nibbles.
pub fn split_u6_into_nibbles(v: u6) -> (u32, u32) {
    let val: u32 = v.into();
    (
        (val >> 4) & 0xF,
         val       & 0xF,
    )
}
