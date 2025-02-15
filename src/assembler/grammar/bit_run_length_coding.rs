use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BitRunLengthCoding(Vec<(char, usize)>);

impl BitRunLengthCoding {
    pub fn get(&self) -> &Vec<(char, usize)> {
        &self.0
    }
}

impl FromStr for BitRunLengthCoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED_LEN: usize = 32;

        let s = s.replace(" ", "");

        if s.len() != EXPECTED_LEN {
            return Err(format!("Input string must be {} characters long", EXPECTED_LEN));
        }

        let mut sections = Vec::new();
        let mut current_char = s.chars().last().unwrap();
        let mut occurences = 0;

        for (rev_idx, char) in s.chars().enumerate() {
            // Check for unusable characters
            if char != '0' && char != '1' && !char.is_ascii_uppercase() {
                if char == ' ' {
                    continue;
                } else {
                    let true_index = EXPECTED_LEN - rev_idx - 1; // rev_idx starts at EXPECTED_LEN - 1 and goes towards 0
                    return Err(format!("Invalid character `{}` at index {}", char, true_index));
                }
            }

            if char == current_char {
                occurences += 1;
            } else {
                sections.push((current_char, occurences));
                current_char = char;
                occurences = 1;
            }
        }

        sections.push((current_char, occurences));

        Ok(Self(sections))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_run_length_coding() {
        println!("{:?}", "0100 AAAA BBBB CCCC 0000 0000 0000 0000".parse::<BitRunLengthCoding>().unwrap());
    }
}
