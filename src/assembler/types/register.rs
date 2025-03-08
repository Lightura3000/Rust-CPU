use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some('r') = value.chars().next() {
            if let Ok(r) = value[1..].parse::<u8>() {
                match r {
                    0 => Ok(Register::R0),
                    1 => Ok(Register::R1),
                    2 => Ok(Register::R2),
                    3 => Ok(Register::R3),
                    4 => Ok(Register::R4),
                    5 => Ok(Register::R5),
                    6 => Ok(Register::R6),
                    7 => Ok(Register::R7),
                    8 => Ok(Register::R8),
                    9 => Ok(Register::R9),
                    10 => Ok(Register::R10),
                    11 => Ok(Register::R11),
                    12 => Ok(Register::R12),
                    13 => Ok(Register::R13),
                    14 => Ok(Register::R14),
                    15 => Ok(Register::R15),
                    _ => Err(()),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_into_register() {
        assert_eq!(Register::from_str("r0"), Ok(Register::R0));
        assert_eq!(Register::from_str("r1"), Ok(Register::R1));
        assert_eq!(Register::from_str("r2"), Ok(Register::R2));
        assert_eq!(Register::from_str("r3"), Ok(Register::R3));
        assert_eq!(Register::from_str("r4"), Ok(Register::R4));
        assert_eq!(Register::from_str("r5"), Ok(Register::R5));
        assert_eq!(Register::from_str("r6"), Ok(Register::R6));
        assert_eq!(Register::from_str("r7"), Ok(Register::R7));
        assert_eq!(Register::from_str("r8"), Ok(Register::R8));
        assert_eq!(Register::from_str("r9"), Ok(Register::R9));
        assert_eq!(Register::from_str("r10"), Ok(Register::R10));
        assert_eq!(Register::from_str("r11"), Ok(Register::R11));
        assert_eq!(Register::from_str("r12"), Ok(Register::R12));
        assert_eq!(Register::from_str("r13"), Ok(Register::R13));
        assert_eq!(Register::from_str("r14"), Ok(Register::R14));
        assert_eq!(Register::from_str("r15"), Ok(Register::R15));

        assert!(Register::from_str("13").is_err());
        assert!(Register::from_str("rtv").is_err());
        assert!(Register::from_str("").is_err());
        assert!(Register::from_str("0").is_err());
        assert!(Register::from_str("r").is_err());
    }
}
