use std::io;
use tools::{is_whitespace,is_number};
use pnm::pbm;
use Image;
use ImageType;
use Mode;
use WordSize;

#[derive(PartialEq, Eq, Debug)]
enum Format {
    PBM,
    PGM,
    PPM,
}

pub fn decode(dat:&Vec<u8>) -> Result<Image, io::Error> {
    let format = match dat[1] {
        b'1' => Format::PBM,
        b'2' => Format::PGM,
        b'3' => Format::PPM,
        b'4' => Format::PBM,
        b'5' => Format::PGM,
        b'6' => Format::PPM,
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
                if format == Format::PBM && header_part > 1 {
                    break;
                } else if (format == Format::PGM || format == Format::PPM) && header_part > 2 {
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

    let word_size = if bit_size > 255 {
        WordSize::SIXTEEN
    } else {
        WordSize::EIGHT
    };

    let final_dat = if format == Format::PBM {
        if image_mode == Mode::ASCII {
            pbm::load_ascii(&dat[data_start..dat.len()])
        } else {
            pbm::load_binary(&dat[data_start..dat.len()],width)
        }
    } else {
        if image_mode == Mode::ASCII {
            vec![]
        } else {
            dat[data_start..dat.len()].to_vec()
        }
    };

    let (image_type,image_depth) = match format {
        Format::PBM => (ImageType::BLACKANDWHITE,1),
        Format::PGM => (ImageType::GRAYSCALE,2),
        Format::PPM => (ImageType::RGB,3),
    };

    Ok(Image {width:width, height:height, image_type:image_type, depth:image_depth, word_size:word_size, data:final_dat})
}
