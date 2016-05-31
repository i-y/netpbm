extern crate netbpm;
use netbpm::ppm::{PPMEncoder,PPMDecoder};
use netbpm::{Mode,BitDepth};
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

const J:[u8;180] = [255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    0,0,0,         255,255,255,   255,255,255,   255,255,255,   0,0,0,   255,255,255,
                    255,255,255,   0,0,0,         0,0,0,         0,0,0,         255,255,255,  255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   255,255,255,  255,255,255,
                    255,255,255,   255,255,255,   255,255,255,   255,255,255,   255,255,255,  255,255,255];

const J_DOUBLE:[u8;360] = [255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           0,0,0,0,0,0,               255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   0,0,0,0,0,0,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   0,0,0,0,0,0,               0,0,0,0,0,0,          0,0,0,0,0,0,         255,255,255,255,255,255,  255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,
                           255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,   255,255,255,255,255,255,];

fn test_arrs(size:u32, a:&[u8], b:&[u8]) -> bool {
    let mut ret = true;
    for i in 0..size {
        ret = ret && (a[i as usize] == b[i as usize]);
    }
    ret
}

#[test]
fn ppm_j_single_ascii() {
    let mut encoder = PPMEncoder::new("test_0a.ppm");
    let enc_result = encoder.save(&J, 6, 10, Mode::ASCII, BitDepth::EIGHT);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PPMDecoder::new("test_0a.ppm");
    let image = decoder.load().unwrap();
    assert_eq!(6, image.width);
    assert_eq!(10, image.height);
    assert_eq!(BitDepth::EIGHT, image.depth);
    assert!(test_arrs(180, &image.dat, &J));
    let _ = fs::remove_file("test_0a.ppm");
}

#[test]
fn ppm_j_single_binary() {
    let mut encoder = PPMEncoder::new("test_0b.ppm");
    let enc_result = encoder.save(&J, 6, 10, Mode::BINARY, BitDepth::EIGHT);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PPMDecoder::new("test_0b.ppm");
    let image = decoder.load().unwrap();
    assert_eq!(6, image.width);
    assert_eq!(10, image.height);
    assert_eq!(BitDepth::EIGHT, image.depth);
    assert!(test_arrs(180, &image.dat, &J));
    let _ = fs::remove_file("test_0b.ppm");
}

#[test]
fn ppm_j_double_ascii_error() {
    let mut encoder = PPMEncoder::new("test_1a.ppm");
    let enc_result = encoder.save(&J_DOUBLE, 6, 10, Mode::ASCII, BitDepth::SIXTEEN);
    match enc_result {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Width can not be greater than 70 characters for ascii ppm files."),
    }
    let _ = fs::remove_file("test_1a.ppm");
}

#[test]
fn ppm_j_double_binary() {
    let mut encoder = PPMEncoder::new("test_1b.ppm");
    let enc_result = encoder.save(&J_DOUBLE, 6, 10, Mode::BINARY, BitDepth::SIXTEEN);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PPMDecoder::new("test_1b.ppm");
    let image = decoder.load().unwrap();
    assert_eq!(6, image.width);
    assert_eq!(10, image.height);
    assert_eq!(BitDepth::SIXTEEN, image.depth);
    assert!(test_arrs(360, &image.dat, &J_DOUBLE));
    let _ = fs::remove_file("test_1b.ppm");
}

#[test]
fn ppm_wrong_filetype_error() {
    let mut file = File::create("file_type.ppm").unwrap();
    let _ = file.write(b"P2\n2 2\n255\n0 255\n255 0");
    let mut decoder = PPMDecoder::new("file_type.ppm");
    let dec_result = decoder.load();
    match dec_result {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Input file is not a ppm file."),
    }
    let _ = fs::remove_file("file_type.ppm");
}
