#[derive(Debug, Copy, Clone)]
pub struct U2(u8);

#[allow(dead_code)]
impl U2 {
    pub const MAX: u8 = 4;

    pub fn new(value: u8) -> Option<U2> {
        if value < 4 {
            Some( U2(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct U3(u8);

#[allow(dead_code)]
impl U3 {
    pub const MAX: u8 = 8;

    pub fn new(value: u8) -> Option<U3> {
        if value < 8 {
            Some( U3(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct U6(u8);

#[allow(dead_code)]
impl U6 {
    pub const MAX: u8 = 64;

    pub fn new(value: u8) -> Option<U6> {
        if value < 64 {
            Some( U6(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}
