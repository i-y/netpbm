extern crate netbpm;
use netbpm::pbm::{PBMEncoder,PBMDecoder};
use netbpm::Mode;
use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// The following constants are designed to test three case:
//   - The width is less than a byte (8)
//   - The width is greater than a byte but not a multiple of 8.
//   - The width is greater than a byte and is a multiple of 8.

// 6 10
const J:[u8;60] = [0,0,0,0,1,0,
                   0,0,0,0,1,0,
                   0,0,0,0,1,0,
                   0,0,0,0,1,0,
                   0,0,0,0,1,0,
                   0,0,0,0,1,0,
                   1,0,0,0,1,0,
                   0,1,1,1,0,0,
                   0,0,0,0,0,0,
                   0,0,0,0,0,0];
// 10 9
const F:[u8;90] = [1,1,1,1,1,1,1,1,1,1,
                   1,1,1,1,1,1,1,1,1,1,
                   1,1,0,0,0,0,0,0,0,0,
                   1,1,0,0,0,0,0,0,0,0,
                   1,1,1,1,1,1,0,0,0,0,
                   1,1,1,1,1,1,0,0,0,0,
                   1,1,0,0,0,0,0,0,0,0,
                   1,1,0,0,0,0,0,0,0,0,
                   1,1,0,0,0,0,0,0,0,0];
// 16, 14
const H:[u8;224] = [1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1,
                    1,1,1,1,0,0,0,0,0,0,0,0,1,1,1,1];

pub fn test_arrs(size:u8, a:&[u8], b:&[u8]) -> bool {
    let mut ret = true;
    for i in 0..size {
        ret = ret && (a[i as usize] == b[i as usize]);
    }
    ret
}

#[test]
fn pbm_j_ascii() {
    let mut encoder = PBMEncoder::new("test_0a.pbm");
    let enc_result = encoder.save(&J, 6, 10, Mode::ASCII);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PBMDecoder::new("test_0a.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(6, image.width);
    assert_eq!(10, image.height);
    assert!(test_arrs(60, &image.dat, &J));
    let _ = fs::remove_file("test_0a.pbm");
}

#[test]
fn pbm_j_binary() {
    let mut encoder = PBMEncoder::new("test_0b.pbm");
    let enc_result = encoder.save(&J, 6, 10, Mode::BINARY);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PBMDecoder::new("test_0b.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(6, image.width);
    assert_eq!(10, image.height);
    assert!(test_arrs(60, &image.dat, &J));
    let _ = fs::remove_file("test_0b.pbm");
}

#[test]
fn pbm_f_ascii() {
    let mut encoder = PBMEncoder::new("test_1a.pbm");
    let enc_result = encoder.save(&F, 10, 9, Mode::ASCII);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PBMDecoder::new("test_1a.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(10, image.width);
    assert_eq!(9, image.height);
    assert!(test_arrs(90, &image.dat, &F));
    let _ = fs::remove_file("test_1a.pbm");
}

#[test]
fn pbm_f_binary() {
    let mut encoder = PBMEncoder::new("test_1b.pbm");
    let enc_result = encoder.save(&F, 10, 9, Mode::BINARY);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PBMDecoder::new("test_1b.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(10, image.width);
    assert_eq!(9, image.height);
    assert!(test_arrs(90, &image.dat, &F));
    let _ = fs::remove_file("test_1b.pbm");
}

#[test]
fn pbm_h_ascii() {
    let mut encoder = PBMEncoder::new("test_2a.pbm");
    let enc_result = encoder.save(&H, 16, 14, Mode::ASCII);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PBMDecoder::new("test_2a.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(16, image.width);
    assert_eq!(14, image.height);
    assert!(test_arrs(224, &image.dat, &H));
    let _ = fs::remove_file("test_2a.pbm");
}

#[test]
fn pbm_h_binary() {
    let mut encoder = PBMEncoder::new("test_2b.pbm");
    let enc_result = encoder.save(&H, 16, 14, Mode::BINARY);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut decoder = PBMDecoder::new("test_2b.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(16, image.width);
    assert_eq!(14, image.height);
    assert!(test_arrs(224, &image.dat, &H));
    let _ = fs::remove_file("test_2b.pbm");
}

#[test]
fn pbm_ascii_width_oversize() {
    let mut encoder = PBMEncoder::new("test_3.pbm");
    let enc_result = encoder.save(&H, 160, 14, Mode::ASCII);
    match enc_result {
        Ok(()) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Width can not be greater than 70 for ascii pbm files."),
    }
    let _ = fs::remove_file("test_3.pbm");
}

#[test]
fn pbm_ascii_commented() {
    let mut file = File::create("test_4.pbm").unwrap();
    let _ = file.write_fmt(format_args!("P1\n#This is a comment.\n{} {}\n1 0\n0 1",2,2));
    let mut decoder = PBMDecoder::new("test_4.pbm");
    let image = decoder.load().unwrap();
    assert_eq!(2, image.width);
    assert_eq!(2, image.height);
    let dat = [1,0,0,1];
    assert!(test_arrs(4, &image.dat, &dat));
    let _ = fs::remove_file("test_4.pbm");
}

#[test]
fn pbm_not_netbpm_file() {
    let mut file = File::create("test_5.pbm").unwrap();
    let _ = file.write_fmt(format_args!("M1\n#This is a comment.\n{} {}\n1 0\n0 1",2,2));
    let mut decoder = PBMDecoder::new("test_5.pbm");
    let dec_result = decoder.load();
    match dec_result {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Input file is not a netbpm file."),
    }
    let _ = fs::remove_file("test_5.pbm");
}

#[test]
fn pbm_not_pbm_file() {
    let mut file = File::create("test_6.pbm").unwrap();
    let _ = file.write_fmt(format_args!("P9\n#This is a comment.\n{} {}\n1 0\n0 1",2,2));
    let mut decoder = PBMDecoder::new("test_6.pbm");
    let dec_result = decoder.load();
    match dec_result {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Input file is not a netbpm file."),
    }
    let _ = fs::remove_file("test_6.pbm");
}

#[test]
fn pbm_bad_header() {
    let mut file = File::create("test_7.pbm").unwrap();
    let _ = file.write_fmt(format_args!("P1\n#This is a comment.\n{} b{}\n1 0\n0 1",2,2));
    let mut decoder = PBMDecoder::new("test_7.pbm");
    let dec_result = decoder.load();
    match dec_result {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Unexpected character in file header. Character: 98"),
    }
    let _ = fs::remove_file("test_7.pbm");
}
