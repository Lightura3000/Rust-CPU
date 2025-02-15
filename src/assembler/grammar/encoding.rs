use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoding(HashMap<char, usize>);

impl Encoding {
    pub fn new(encoding: Vec<(char, usize)>) -> Option<Self> {
        let unique_elements = encoding.iter().collect::<HashSet<_>>().len();

        if unique_elements != encoding.len() {
            return None
        }

        let mut map = HashMap::with_capacity(encoding.len());

        for (c, n) in encoding {
            map.insert(c, n);
        }

        Some(Encoding(map))
    }

    pub fn get(&self, c: char) -> Option<&usize> {
        self.0.get(&c)
    }
}
