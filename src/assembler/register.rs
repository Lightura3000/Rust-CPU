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

/// ```
/// let reg = Register::try_from("r5").unwrap();
/// ```
impl TryFrom<&str> for Register {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
                    _ => Err(format!("Unknown register: '{}'", value)),
                }
            } else {
                Err(format!("Failed to parse register number: '{}'", value))
            }
        } else {
            Err(format!("Register must start with 'r': '{}'", value))
        }
    }
}

/// ```
/// use std::str::FromStr;
/// let reg = "r10".parse::<Register>().unwrap();
/// ```
impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Register::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_into_register() {
        assert!(TryInto::<Register>::try_into("r0").is_ok());
        assert!(TryInto::<Register>::try_into("r1").is_ok());
        assert!(TryInto::<Register>::try_into("r2").is_ok());
        assert!(TryInto::<Register>::try_into("r3").is_ok());
        assert!(TryInto::<Register>::try_into("r4").is_ok());
        assert!(TryInto::<Register>::try_into("r5").is_ok());
        assert!(TryInto::<Register>::try_into("r6").is_ok());
        assert!(TryInto::<Register>::try_into("r7").is_ok());
        assert!(TryInto::<Register>::try_into("r8").is_ok());
        assert!(TryInto::<Register>::try_into("r9").is_ok());
        assert!(TryInto::<Register>::try_into("r10").is_ok());
        assert!(TryInto::<Register>::try_into("r11").is_ok());
        assert!(TryInto::<Register>::try_into("r12").is_ok());
        assert!(TryInto::<Register>::try_into("r13").is_ok());
        assert!(TryInto::<Register>::try_into("r14").is_ok());
        assert!(TryInto::<Register>::try_into("r15").is_ok());

        assert!(TryInto::<Register>::try_into("13").is_err());
        assert!(TryInto::<Register>::try_into("rtv").is_err());
        assert!(TryInto::<Register>::try_into("").is_err());
        assert!(TryInto::<Register>::try_into("0").is_err());
        assert!(TryInto::<Register>::try_into("r").is_err());
    }
}
