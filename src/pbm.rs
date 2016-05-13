//! A module to provide for saving and loading images in the Portable BitMap format.
//!
//! The pbm (Portable BitMap) format is black and white only and all pixels are either 0 or 1.
//!
//! # Examples
//!
//! ```
//! # use std::fs;
//! use netbpm::pbm::{PBMEncoder,PBMDecoder};
//! use netbpm::{Mode,Image};
//!
//! let dat:[u8;4] = [1,0,0,1];
//! let mut encoder = PBMEncoder::new("test_file.pbm");
//! encoder.save(&dat, 2, 2, Mode::ASCII).unwrap();
//!
//! let mut decoder = PBMDecoder::new("test_file.pbm");
//! let image = decoder.load().unwrap();
//! # let _ = fs::remove_file("test_file.pbm");
//! ```

use std::io;
use std::fs::File;
use std::io::prelude::*;
use tools::{get_header, ImageType};
use Mode;
use Image;

/// Encodes an image as a pbm file.
pub struct PBMEncoder {
    f: File,
}

/// Decodes an image in the pbm format.
pub struct PBMDecoder {
    f: File,
}

impl PBMEncoder {
    /// Create a new `PBMEncoder`
    ///
    /// Creates a new `PBMEncoder` by creating a file with the specified name. The file extension
    /// is not important for using this module to encode/decode pbm images.
    ///
    /// # Examples
    ///
    /// ```
    ///  # use std::fs;
    /// use netbpm::pbm::PBMEncoder;
    ///
    /// let encoder = PBMEncoder::new("pbm_file.pbm");
    /// # let _ = fs::remove_file("pbm_file.pbm");
    /// ```
    pub fn new(file_name: &str) -> PBMEncoder {
        let file = File::create(file_name).unwrap();
        PBMEncoder{f : file}
    }

    /// Saves image data to the file stored by the `PBMEncoder`.
    ///
    /// This method will record image data to the file. It takes a slice with the data as bytes,
    /// the width, the height, and the `Mode`. For the purpose of the pbm file format any byte
    /// value greater than 0 will be recorded as 1 in the source.
    ///
    /// # Examples
    ///
    /// ```
    /// use netbpm::pbm::PBMEncoder;
    /// use netbpm::Mode;
    ///
    /// // This will save a pbm file in the form of a J and in an ASCII format.
    /// const data:[u8;60] = [0,0,0,0,1,0,
    ///                       0,0,0,0,1,0,
    ///                       0,0,0,0,1,0,
    ///                       0,0,0,0,1,0,
    ///                       0,0,0,0,1,0,
    ///                       0,0,0,0,1,0,
    ///                       1,0,0,0,1,0,
    ///                       0,1,1,1,0,0,
    ///                       0,0,0,0,0,0,
    ///                       0,0,0,0,0,0];
    /// let mut encoder = PBMEncoder::new("test_file.pbm");
    /// encoder.save(&data, 6, 10, Mode::ASCII).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Aside from the regular errors associated with file access this function will throw an
    /// error if the user attempts to save an image more than 70 pixels wide in ASCII mode.
    pub fn save(&mut self, dat: &[u8], height: u32, width: u32, mode: Mode) -> Result<(), io::Error> {
        Ok(
            match mode {
                Mode::ASCII => try!(self.save_ascii(dat, height, width)),
                Mode::BINARY => try!(self.save_binary(dat, height, width)),
            }
        )
    }

    /// Saves a pbm file in ASCII format.
    ///
    /// # Examples
    ///
    /// P1
    /// # This is an example bitmap of the letter "J"
    /// 6 10
    /// 0 0 0 0 1 0
    /// 0 0 0 0 1 0
    /// 0 0 0 0 1 0
    /// 0 0 0 0 1 0
    /// 0 0 0 0 1 0
    /// 0 0 0 0 1 0
    /// 1 0 0 0 1 0
    /// 0 1 1 1 0 0
    /// 0 0 0 0 0 0
    /// 0 0 0 0 0 0
    ///
    /// P1 = file's magic number
    /// # foo = comment
    /// num num = width and height (in that order)
    fn save_ascii(&mut self, dat: &[u8], width: u32, height: u32) -> Result<(), io::Error> {
        // In theory we can ignore this with no downside but it would no longer be conformant.
        if width > 70 {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Width can not be greater than 70 for ascii pbm files."));
        }
        // write the header information.
        try!(self.f.write_fmt(format_args!("P1\n{} {}\n",width,height)));
        // track if we need to insert a new line character at the end of a row.
        let mut nl = 1;
        // write the actual image data.
        for val in dat {
            match *val {
                0 => try!(self.f.write_all(b"0")),
                _ => try!(self.f.write_all(b"1")),
            };
            if nl < width {
                try!(self.f.write_all(b" "));
                nl += 1;
            } else {
                try!(self.f.write_all(b"\n"));
                nl = 1;
            }
        }
        Ok(())
    }

    /// saves a pbm file in binary format
    ///
    /// The header is actually still in ASCII and is the same as the header above, except with
    /// a magic number of P4
    ///
    /// The pixels are stored as single bits with "don't care" bits added as padding to a byte at
    /// the end of the row. The data shown above would look like this in binary. White space has
    /// been added for clarity but there are no spaces or new lines in the actual data.
    ///
    /// 0000 1000
    /// 0000 1000
    /// 0000 1000
    /// 0000 1000
    /// 0000 1000
    /// 0000 1000
    /// 1000 1000
    /// 0111 0000
    /// 0000 0000
    /// 0000 0000
    fn save_binary(&mut self, dat: &[u8], width: u32, height: u32) -> Result<(), io::Error> {
        // write the header
        try!(self.f.write_fmt(format_args!("P4\n{} {}\n",width,height)));
        // If our width is not a multiple of 8 we will need to add some "don't care" bits to
        // the final byte in each row.
        let padding = 8 - (width % 8);
        // New Line tracker. We don't actually add a new line character but we need to add any
        // padding to the byte at the end of a row
        let mut nl = 1;
        // tracks if we should start a new byte
        let mut byte_pos = 0;
        let mut tmp:[u8;1] = [0];
        // write our image data.
        for val in dat {
            tmp[0] = tmp[0] << 1;
            tmp[0] += match *val {
                0 => 0,
                _ => 1,
            };
            byte_pos += 1;
            // If we've reached the end of a byte
            if byte_pos == 8 {
                try!(self.f.write(&tmp));
                tmp[0] = 0;
                byte_pos = 0;
            }
            // If we haven't reached the end of a row, keep going.
            // If we have reached the end of a row, add any padding bits to the final byte and then
            // write it to the file.
            if nl < width {
                nl += 1;
            } else {
                // Only write a padded byte if the width is not a multiple of 8.
                if byte_pos != 0 {
                    tmp[0] = tmp[0] << padding;
                    try!(self.f.write(&tmp));
                }
                nl = 1;
                byte_pos = 0;
            }
        }
        Ok(())
    }
}

impl PBMDecoder {
    /// Create a new `PBMDecoder`
    ///
    /// Creates a new `PBMDecoder` that reads from the specified file. The file extension does not
    /// matter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// # use std::fs::File;
    /// use netbpm::pbm::PBMDecoder;
    ///
    /// # let _ = File::create("saved_file.pbm");
    /// let decoder = PBMDecoder::new("saved_file.pbm");
    ///  # let _ = fs::remove_file("saved_file.pbm");
    /// ```
    pub fn new(file_name: &str) -> PBMDecoder {
        let file = File::open(file_name).unwrap();
        PBMDecoder{f : file}
    }

    /// Loads a pbm file.
    ///
    /// Will load a pbm file that's in either ASCII or binary format. This particular method will
    /// parse the header information before handing it off to more specalized methods to load the
    /// binary or ASCII data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// use netbpm::pbm::PBMDecoder;
    /// use netbpm::Image;
    ///
    /// # let mut file = File::create("image.pbm").unwrap();
    /// # let _ = file.write(b"P1\n2 2\n1 0\n0 1");
    /// let mut decoder = PBMDecoder::new("image.pbm");
    /// let image = decoder.load().unwrap();
    /// # let _ = fs::remove_file("image.pbm");
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return all general file IO errors that can be raised by file read
    /// operations. Additionally, it will return an error if the magic number does not start with
    /// P, if the magic number is not P1 or P4, or if there is a non-numeric of whitespace
    /// character in the size line of the header.
    pub fn load(&mut self) ->  Result<Image, io::Error> {
        let mut all_data:Vec<u8> = vec![];
        try!(self.f.read_to_end(&mut all_data));
        let header = try!(get_header(&all_data));

        // check the magic number.
        if header.image_type != ImageType::PBM {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file is not a pbm file."));
        }

        match header.mode {
            Mode::ASCII => Ok(Image{width:header.width, height:header.height,
                           dat:self.load_ascii(&all_data[header.dat_start..all_data.len()]),
                           depth: header.depth}),
            Mode::BINARY => Ok(Image{width:header.width, height:header.height,
                           dat:self.load_binary(&all_data[header.dat_start..all_data.len()], header.width),
                           depth: header.depth})
        }
    }

    /// Load image data stored in ASCII format.
    fn load_ascii(&self, inpt: &[u8]) -> Vec<u8> {
        let mut vals:Vec<u8> = vec![];
        for x in inpt{
            if *x == 48 {
                vals.push(0);
            } else if *x == 49 {
                vals.push(1);
            }
        }
        vals
    }

    /// Load image data stored in binary format.
    fn load_binary(&self, inpt: &[u8], width: u32) -> Vec<u8> {
        let mut vals:Vec<u8> = vec![];
        // Keep track of what index we're on, counted in bits.
        let mut ind = 8;
        // How much padding (if any) exists on bytes at the end of a row.
        let padding = ((8 - (width % 8)) % 8) as u8;
        // Use a mask to pick up individual bits.
        let masks:[u8;8] = [128, 64, 32, 16, 8, 4, 2, 1];
        for x in inpt {
            for i in 0..8 {
                let mv:u8 = 8 - i;
                let v:u8 = (x & masks[i as usize]) >> (mv - 1);
                if ind < width || mv > padding {
                    vals.push(v);
                }
            }
            if ind >= width {
                ind = 8;
            } else {
                ind += 8;
            }
        }
        vals
    }
}
