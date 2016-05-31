use std::io;
use std::fs::File;
use std::io::prelude::*;
use tools::{get_header, ImageType};
use Mode;
use Image;
use BitDepth;

/// Encodes an image as a ppm file.
pub struct PPMEncoder {
    f: File,
}

/// Decodes an image in the pgm format.
pub struct PPMDecoder {
    f: File,
}

impl PPMEncoder {
    /// Create a new `PPMEncoder`
    ///
    /// Creates a new `PPMEncoder` by creating a file with the specified name. The file extension
    /// is not important for using this module to encode/decode ppm images.
    ///
    /// # Examples
    ///
    /// ```
    ///  # use std::fs;
    /// use netbpm::ppm::PPMEncoder;
    ///
    /// let encoder = PPMEncoder::new("ppm_file.ppm");
    /// # let _ = fs::remove_file("ppm_file.ppm");
    /// ```
    pub fn new(file_name: &str) -> PPMEncoder {
        let file = File::create(file_name).unwrap();
        PPMEncoder{f : file}
    }

    /// Saves image data to the file stored by the `PPMEncoder`.
    ///
    /// This method will record image data to the file. It takes a slice with the data as bytes,
    /// the width, the height, the `Mode`, and the `BitDepth`. The variable bit depth means that
    /// the maximum value is either 255 or 65,535. Note that in this format 0 is black and the max
    /// value allowed by the bit depth is white.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// use netbpm::ppm::PPMEncoder;
    /// use netbpm::{Mode,BitDepth};
    ///
    /// // This will save an image of a `J` in an eight-bit, ascii ppm file
    /// const data:[u8;180] = [255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        0,0,0,         255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
    ///                        255,255,255,   0,0,0,         0,0,0,         0,0,0,         255,255,255,  255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   255,255,255,  255,255,255,
    ///                        255,255,255,   255,255,255,   255,255,255,   255,255,255,   255,255,255,  255,255,255];
    ///
    /// let mut encoder = PPMEncoder::new("test_file.ppm");
    /// encoder.save(&data, 6, 10, Mode::ASCII, BitDepth::EIGHT).unwrap();
    /// # let _ = fs::remove_file("test_file.ppm");
    /// ```
    ///
    /// # Errors
    ///
    /// Aside from the regular errors associated with file access this function will throw an
    /// error if the user attempts to save an image more than 70 characters wide in ASCII mode.
    /// Note that because it is characters and not pixels you will frequently only be able to save
    /// fewer than 70 pixels, especially in 16-bit mode.
    pub fn save(&mut self, dat: &[u8], height: u32, width: u32, mode: Mode, depth: BitDepth) -> Result<(), io::Error> {
        Ok(
            match mode {
                Mode::ASCII => try!(self.save_ascii(dat, height, width, depth)),
                Mode::BINARY => try!(self.save_binary(dat, height, width, depth)),
            }
        )
    }

    /// Saves a ppm file in ascii format.
    ///
    /// This method saves the data in ascii format. This is a bit tricky as the ints needs to be
    /// converted to strings while at the same time keeping track of the total number of characters
    /// on the line to prevent more than 70 being saved.
    ///
    /// TODO: Double-check that we're not missing a default API call which would make this easier.
    fn save_ascii(&mut self, dat: &[u8], width: u32, height: u32, depth: BitDepth) -> Result<(), io::Error> {
        // In theory we can ignore this with no downside but it would no longer be conformant.
        if width > 70 {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Width can not be greater than 70 for ascii pgm files."));
        }
        // write the header information.
        try!(self.f.write_fmt(format_args!("P3\n{} {}\n",width,height)));
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
            for j in 0..(width * 3) {
                let val:u16 = match depth {
                    BitDepth::EIGHT => dat[((i * (width * 3)) + j) as usize] as u16,
                    BitDepth::SIXTEEN => {
                        let ind:usize = (((i * (width * 3)) + j) * 2) as usize;
                        ((dat[ind] as u16) << 8) + dat[ind] as u16
                    },
                };
                let v = val.to_string();
                counter = counter + v.len();
                if counter > 70 {
                    return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Width can not be greater than 70 characters for ascii ppm files."));
                }
                try!(self.f.write_all(&v.into_bytes()));
                if j < (width * 3)-1 {
                    try!(self.f.write_all(b" "));
                }
            }
            counter = 0;
            try!(self.f.write_all(b"\n"));
        }

        Ok(())
    }

    /// Save a ppm file in binary format.
    ///
    /// Saving in binary is much easier than in ascii as, after we construct the header, we can
    /// just write the input data directly.
    fn save_binary(&mut self, dat: &[u8], width: u32, height: u32, depth: BitDepth) -> Result<(), io::Error> {
        // write the header
        try!(self.f.write_fmt(format_args!("P6\n{} {}\n",width,height)));
        match depth {
            BitDepth::EIGHT => try!(self.f.write_all(b"255\n")),
            BitDepth::SIXTEEN => try!(self.f.write_all(b"65535\n")),
        }
        // write the image data
        try!(self.f.write_all(dat));
        Ok(())
    }
}

impl PPMDecoder {
    /// Create a new `PPMDecoder`
    ///
    /// Creates a new `PPMDecoder` that reads from the specified file. The file extension does not
    /// matter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// # use std::fs::File;
    /// use netbpm::ppm::PPMDecoder;
    ///
    /// # let _ = File::create("saved_file.ppm");
    /// let decoder = PPMDecoder::new("saved_file.ppm");
    ///  # let _ = fs::remove_file("saved_file.ppm");
    /// ```
    pub fn new(file_name: &str) -> PPMDecoder {
        let file = File::open(file_name).unwrap();
        PPMDecoder{f : file}
    }

    /// Loads a ppm file.
    ///
    /// Will load a ppm file that's in either ASCII or binary format. The file extension does not
    /// matter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// use netbpm::ppm::PPMDecoder;
    /// use netbpm::Image;
    ///
    /// # let mut file = File::create("image.ppm").unwrap();
    /// # let _ = file.write(b"P3\n2 2\n255\n0 0 0 255 255 255\n255 255 255 0 0 0");
    /// let mut decoder = PPMDecoder::new("image.ppm");
    /// let image = decoder.load().unwrap();
    /// # let _ = fs::remove_file("image.ppm");
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return all general file IO errors that can be raised by file read
    /// operations. Additionally, it will return an error if the image type is not ppm as well
    /// as all file header parsing errors.
    pub fn load(&mut self) ->  Result<Image, io::Error> {
        let mut all_data:Vec<u8> = vec![];
        try!(self.f.read_to_end(&mut all_data));
        let header = try!(get_header(&all_data));

        // check the magic number.
        if header.image_type != ImageType::PPM {
            return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file is not a ppm file."));
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
