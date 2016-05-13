use std::io;
use std::fs::File;
use std::io::prelude::*;
use tools::{get_header, ImageType};
use Mode;
use Image;
use BitDepth;


/// Encodes an image as a pgm file.
pub struct PGMEncoder {
    f: File,
}

/// Decodes an image in the pgm format.
pub struct PGMDecoder {
    f: File,
}

impl PGMEncoder {
    /// Create a new `PGMEncoder`
    ///
    /// Creates a new `PGMEncoder` by creating a file with the specified name. The file extension
    /// is not important for using this module to encode/decode pbm images.
    ///
    /// # Examples
    ///
    /// ```
    ///  # use std::fs;
    /// use netbpm::pgm::PGMEncoder;
    ///
    /// let encoder = PGMEncoder::new("pgm_file.pgm");
    /// # let _ = fs::remove_file("pgm_file.pgm");
    /// ```
    pub fn new(file_name: &str) -> PGMEncoder {
        let file = File::create(file_name).unwrap();
        PGMEncoder{f : file}
    }

    pub fn save(&mut self, dat: &[u8], height: u32, width: u32, mode: Mode, depth: BitDepth) -> Result<(), io::Error> {
        Ok(
            match mode {
                Mode::ASCII => try!(self.save_ascii(dat, height, width, depth)),
                Mode::BINARY => try!(self.save_binary(dat, height, width, depth)),
            }
        )
    }

    fn save_ascii(&mut self, dat: &[u8], width: u32, height: u32, depth: BitDepth) -> Result<(), io::Error> {
        // In theory we can ignore this with no downside but it would no longer be conformant.
        if width > 70 {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Width can not be greater than 70 for ascii pgm files."));
        }
        // write the header information.
        try!(self.f.write_fmt(format_args!("P2\n{} {}\n",width,height)));
        match depth {
            BitDepth::EIGHT => try!(self.f.write_all(b"255\n")),
            BitDepth::SIXTEEN => try!(self.f.write_all(b"65535\n")),
        }

        // track if we need to insert a new line character at the end of a row.
        //let mut nl = 1;
        // track the number of characters in the line.
        let mut counter = 0;
        // write the actual image data.
        for i in 0..height {
            for j in 0..width {
                let val:u16 = match depth {
                    BitDepth::EIGHT => dat[((i * width) + j) as usize] as u16,
                    BitDepth::SIXTEEN => {
                        let ind:usize = (((i * width) + j) * 2) as usize;
                        ((dat[ind] as u16) << 8) + dat[ind] as u16
                    },
                };
                let v = val.to_string();
                counter = counter + v.len();
                if counter > 70 {
                    return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Width can not be greater than 70 characters for ascii pgm files."));
                }
                try!(self.f.write_all(&v.into_bytes()));
                if j < width-1 {
                    try!(self.f.write_all(b" "));
                }
            }
            counter = 0;
            try!(self.f.write_all(b"\n"));
        }

        Ok(())
    }

    fn save_binary(&mut self, dat: &[u8], width: u32, height: u32, depth: BitDepth) -> Result<(), io::Error> {
        // write the header
        try!(self.f.write_fmt(format_args!("P5\n{} {}\n",width,height)));
        match depth {
            BitDepth::EIGHT => try!(self.f.write_all(b"255\n")),
            BitDepth::SIXTEEN => try!(self.f.write_all(b"65535\n")),
        }
        // write the image data
        try!(self.f.write_all(dat));
        Ok(())
    }
}

impl PGMDecoder {
    /// Create a new `PGMDecoder`
    ///
    /// Creates a new `PGMDecoder` that reads from the specified file. The file extension does not
    /// matter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// # use std::fs::File;
    /// use netbpm::pgm::PGMDecoder;
    ///
    /// # let _ = File::create("saved_file.pgm");
    /// let decoder = PGMDecoder::new("saved_file.pgm");
    ///  # let _ = fs::remove_file("saved_file.pgm");
    /// ```
    pub fn new(file_name: &str) -> PGMDecoder {
        let file = File::open(file_name).unwrap();
        PGMDecoder{f : file}
    }

    /// Loads a pgm file.
    ///
    /// Will load a pgm file that's in either ASCII or binary format. The file extension does not
    /// matter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// use netbpm::pgm::PGMDecoder;
    /// use netbpm::Image;
    ///
    /// # let mut file = File::create("image.pgm").unwrap();
    /// # let _ = file.write(b"P2\n2 2\n255\n0 255\n255 0");
    /// let mut decoder = PGMDecoder::new("image.pgm");
    /// let image = decoder.load().unwrap();
    /// # let _ = fs::remove_file("image.pgm");
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return all general file IO errors that can be raised by file read
    /// operations. Additionally, it will return an error if the image type is not pgm as well
    /// as all file header parsing errors.
    pub fn load(&mut self) ->  Result<Image, io::Error> {
        let mut all_data:Vec<u8> = vec![];
        try!(self.f.read_to_end(&mut all_data));
        let header = try!(get_header(&all_data));

        // check the magic number.
        if header.image_type != ImageType::PGM {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file is not a pgm file."));
        }

        match header.mode {
            Mode::ASCII => Ok(Image{width:header.width, height:header.height,
                           dat:self.load_ascii(&all_data[header.dat_start..all_data.len()], &header.depth),
                           depth: header.depth}),
            // unlike with the pbm file format we can take raw binary data with no processing.
            Mode::BINARY => Ok(Image{width:header.width, height:header.height,
                           dat:all_data[header.dat_start..all_data.len()].to_vec(),
                           depth: header.depth})
        }

    }

    /// Load image data stored in ASCII format.
    fn load_ascii(&self, inpt: &[u8], depth: &BitDepth) -> Vec<u8> {
        let mut vals:Vec<u8> = vec![];
        let mut num:u16 = 0;
        for x in inpt{
            if *x > 47 && *x < 58 {
                num = (num * 10) + (*x - 48) as u16;
            } else {
                if *depth == BitDepth::EIGHT {
                    vals.push(num as u8);
                } else {
                    vals.push((num >> 8) as u8);  // most significant byte first.
                    vals.push((num & 255) as u8); // least significant byte second.
                }
                num = 0;
            }
        }
        vals
    }
}
