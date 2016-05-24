use std::io;
use BitDepth;
use Mode;

pub struct ImageHeader {
    pub width:u32,
    pub height:u32,
    pub dat_start:usize,
    pub image_type: ImageType,
    pub depth: BitDepth,
    pub mode: Mode,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ImageType {
    PBM,
    PGM,
}

/// Finds if the character is a whitespace
///
/// White spaces are blanks, TABs, CRs, and LFs
fn is_whitespace(inpt: u8) -> bool {
    inpt == 9 || inpt == 10 || inpt == 13 || inpt == 32
}

/// Finds if the character is a number.
fn is_number(inpt: u8) -> bool {
    inpt > 47 && inpt < 58
}

/// Reads the header from an input data stream.
///
/// Parses the input data to try to get the image size, file type, and bit depth. The file formats
/// contain a magic number, height and width, optional comments, and bit depth depending on the
/// file. These are seperated by whitespace aside from comments, which are only ended with a
/// newline.
pub fn get_header(dat:&Vec<u8>) ->  Result<ImageHeader, io::Error> {

    // Test that the magic number is valid
    if dat[0] != 80 ||  dat[1] < 49 || dat[1] > 55 {
        return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file is not a netbpm file."));
    }

    // What file format does the magic number say we have
    let image_type = match dat[1] {
        49 => ImageType::PBM,
        50 => ImageType::PGM,
        52 => ImageType::PBM,
        53 => ImageType::PGM,
        _ => return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Input file is an unsupported netbpm type.")),
    };

    // Does the magic number say we're binary or ascii
    let image_mode = if dat[1] > 51 && dat[1] < 55 {
        Mode::BINARY
    } else {
        Mode::ASCII
    };

    // The data values we'll be storing.
    let mut width:u32 = 0;
    let mut height:u32 = 0;
    let mut bit_size:u32 = 0;

    // values we'll use to keep track of where in the header we are.
    let mut data_start:usize = 0; // The index where the pixel data starts.
    let mut header_part = -1;     // What part of the header we're reading.
                                  //         0 = width, 1 = height, 2 = bit size
    let mut skip = false;         // Used to skip comments.

    // Read in our actual data.
    for i in 2..dat.len() {
        data_start += 1;
        if skip && dat[i] == 10 { // comments end only at a LF (newline)
            skip = false;
        } else if !skip {
            if dat[i] == b'#' { // start of comments
                skip = true;
            } else if is_number(dat[i]) { // all data we want to save should be numeric
                match header_part {
                    0 => width = (width * 10) + (dat[i] - 48) as u32,
                    1 => height = (height * 10) + (dat[i] - 48) as u32,
                    2 => bit_size = (bit_size * 10) + (dat[i] - 48) as u32,
                    _ => return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "Loader reading past end of header.")),
                }
            } else if is_whitespace(dat[i]) { // witespace triggers a new part of the header
                header_part += 1;
                // check to see if we've finished reading the header
                if image_type == ImageType::PBM && header_part > 1 {
                    break;
                } else if image_type == ImageType::PGM && header_part > 2 {
                    break;
                }
            } else { // a non-numeric, non-whitespace character outside of a comment is an error.
                return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unexpected character in file header. Character: {}", dat[i])));
            }
        }
    }

    // Move the data start to the point after the last number read in the header. Increment by
    // two to discard the white space following the end of the header.
    data_start += 2;

    let bits = if bit_size > 255 {
        BitDepth::SIXTEEN
    } else {
        BitDepth::EIGHT
    };

    Ok(ImageHeader{width:width, height:height, dat_start:data_start, image_type:image_type, depth:bits, mode:image_mode})
}
