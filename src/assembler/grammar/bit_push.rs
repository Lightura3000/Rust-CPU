pub struct BitPush {
    state: u32,
    filled: usize,
}

impl BitPush {
    pub fn new() -> Self {
        Self {
            state: 0,
            filled: 0,
        }
    }

    pub fn push_ones(&mut self, count: usize) {
        if self.filled + count > u32::BITS as usize {
            panic!("BitPush filled too large");
        }
        let shift = u32::BITS as usize - self.filled - count;
        let mask = if count == u32::BITS as usize {
            u32::MAX
        } else {
            ((1u32 << count) - 1) << shift
        };
        self.state |= mask;
        self.filled += count;
    }

    pub fn push_zeros(&mut self, count: usize) {
        if self.filled + count > u32::BITS as usize {
            panic!("BitPush filled too large");
        }
        self.filled += count;
    }

    pub fn push(&mut self, value: u32, bits: usize) {
        if self.filled + bits > u32::BITS as usize {
            panic!("BitPush filled too large");
        }
        let shift = u32::BITS as usize - self.filled - bits;
        // Mask the value to only take the least significant `bits` bits.
        let masked_value = if bits == u32::BITS as usize {
            value
        } else {
            value & ((1u32 << bits) - 1)
        };
        self.state |= masked_value << shift;
        self.filled += bits;
    }

    pub fn get_value(&self) -> Option<u32> {
        if self.filled == u32::BITS as usize {
            Some(self.state)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_push() {
        let mut pusher = BitPush::new();

        pusher.push(15, 4);  // 1111 at bits 28-31
        pusher.push_ones(3);      // 111 at bits 25-27
        pusher.push_zeros(2);     // 00 at bits 23-24
        pusher.push(5, 3);   // 101 at bits 20-22

        assert_eq!(pusher.state >> 20, 0b1111_111_00_101);
    }

    #[test]
    fn test_mixed_sizes() {
        let mut pusher = BitPush::new();

        pusher.push(63, 6);  // 111111
        pusher.push(2, 2);   // 10
        pusher.push(31, 5);  // 11111
        pusher.push(7, 3);   // 111

        assert_eq!(pusher.state >> 16, 0b111111_10_11111_111);
    }
}