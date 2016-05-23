use std::io;
use std::fs::File;
use std::io::prelude::*;
use Mode;
use Image;
use BitDepth;


/// Encodes an image as a pgm file.
pub struct PGMEncoder {
    f: File,
}

/// Decodes an image in the pgm format.
/*pub struct PGMDecoder {
    f: File,
}*/

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
        /*
        0 0 1 1 2 2
        3 3 4 4 5 5
        6 6 7 7 8 8
        */
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
                println!("{}, {}", val, v);
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
        /*for val in dat {
            let v = val.to_string();
            counter = counter + v.len();
            if counter > 70 {
                return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Width can not be greater than 70 characters for ascii pgm files."));
            }
            try!(self.f.write_all(&v.into_bytes()));
            if nl < width {
                try!(self.f.write_all(b" "));
                nl += 1;
            } else {
                try!(self.f.write_all(b"\n"));
                nl = 1;
                counter = 0;
            }
        }*/
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
