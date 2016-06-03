use std::io;
use std::fs::File;
use std::io::prelude::*;

mod pnm;
mod tools;

#[derive(PartialEq, Eq, Debug)]
pub enum Mode {
    ASCII,
    BINARY,
}

#[derive(PartialEq, Eq, Debug)]
pub enum WordSize {
    EIGHT,
    SIXTEEN,
}

#[derive(PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub enum ImageType {
    BLACKANDWHITE,
    GRAYSCALE,
    RGB,
    BLACKANDWHITE_ALPHA,
    GRAYSCALE_ALPHA,
    RGB_ALPHA,
}


pub struct Image {
    pub width:u32,
    pub height:u32,
    pub image_type: ImageType,
    pub depth: u8,
    pub word_size: WordSize,
    pub data: Vec<u8>,
}


impl Image {
    pub fn new() -> Image {
        Image {width:0, height:0, image_type:ImageType::RGB, depth:0, word_size:WordSize::EIGHT, data:vec![]}
    }

    pub fn load(inpt:&str) -> Result<Image,io::Error> {
        let mut file = File::open(inpt).unwrap();
        let mut all_data:Vec<u8> = vec![];
        try!(file.read_to_end(&mut all_data));

        //let mut img = try!(tools::get_header(&all_data));

        // Test that the magic number is valid
        if all_data[0] != 80 ||  all_data[1] < 49 || all_data[1] > 55 {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file is not a netbpm file."));
        }

        let img = if all_data[0] < b'7' {
            try!(pnm::decode(&all_data))
        } else {
            Image::new()
        };

        Ok(img)
    }
}

/*mod tools;
#[cfg(test)]
mod header_tests;

pub mod pbm;
pub mod pgm;
pub mod ppm;*/
