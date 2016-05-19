#[derive(PartialEq, Eq, Debug)]
pub enum Mode {
    ASCII,
    BINARY,
}

#[derive(PartialEq, Eq, Debug)]
pub enum BitDepth {
    EIGHT,
    SIXTEEN,
}

pub struct Image {
    pub width:u32,
    pub height:u32,
    pub dat: Vec<u8>,
    pub depth: BitDepth,
}

mod tools;
#[cfg(test)]
mod header_tests;

pub mod pbm;
pub mod pgm;
