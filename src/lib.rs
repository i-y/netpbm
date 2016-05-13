pub enum Mode {
    ASCII,
    BINARY,
}

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

pub mod pbm;
