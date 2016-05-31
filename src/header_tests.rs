use BitDepth;
use Mode;
use tools::{get_header,ImageType};
use std::error::Error;

#[test]
fn invalid_magic_number() {
    let dat:Vec<u8> = vec![b'L', b'3'];
    let header = get_header(&dat);
    match header {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Input file is not a netbpm file."),
    }
}

#[test]
fn invalid_magic_number_two() {
    let dat:Vec<u8> = vec![b'P', b'9'];
    let header = get_header(&dat);
    match header {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Input file is not a netbpm file."),
    }
}

#[test]
fn read_header_bad_char() {
    let dat:Vec<u8> = vec![b'P', b'1', 10, b'6', b'?', b' ', b'1', b'0', 10];
    let header = get_header(&dat);
    match header {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.description(),"Unexpected character in file header. Character: 63"),
    }
}

#[test]
fn read_pbm_header_ascii() {
    let dat:Vec<u8> = vec![b'P', b'1', 10, b'6', b' ', b'1', b'0', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 8);
    assert_eq!(header.image_type, ImageType::PBM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pbm_header_binary() {
    let dat:Vec<u8> = vec![b'P', b'4', 10, b'6', b' ', b'1', b'0', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 8);
    assert_eq!(header.image_type, ImageType::PBM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::BINARY);
}

#[test]
fn read_pbm_header_linbreak_whitespace() {
    let dat:Vec<u8> = vec![b'P', b'1', 10, b'6', 10, b'1', b'0', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 8);
    assert_eq!(header.image_type, ImageType::PBM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pbm_header_commented() {
    let dat:Vec<u8> = vec![b'P', b'1', 10, b'#', b'C', b'o', b'm', b'm', b'e', b'n', b't', 10, b'6', b' ', b'1', b'0', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 17);
    assert_eq!(header.image_type, ImageType::PBM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pbm_header_commented_mid_dim() {
    let dat:Vec<u8> = vec![b'P', b'1', 10, b'6', b' ', b'#', b'C', b'o', b'm', b'm', b'e', b'n', b't', 10, b'1', b'0', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 17);
    assert_eq!(header.image_type, ImageType::PBM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pgm_header_ascii_eight() {
    let dat:Vec<u8> = vec![b'P', b'2', 10, b'6', b' ', b'1', b'0', 10, b'2', b'5', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 12);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pgm_header_ascii_sixteen() {
    let dat:Vec<u8> = vec![b'P', b'2', 10, b'6', b' ', b'1', b'0', 10, b'6', b'5', b'5', b'3', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 14);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::SIXTEEN);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pgm_header_binary_eight() {
    let dat:Vec<u8> = vec![b'P', b'5', 10, b'6', b' ', b'1', b'0', 10, b'2', b'5', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 12);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::BINARY);
}

#[test]
fn read_pgm_header_binary_sixteen() {
    let dat:Vec<u8> = vec![b'P', b'5', 10, b'6', b' ', b'1', b'0', 10, b'6', b'5', b'5', b'3', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 14);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::SIXTEEN);
    assert_eq!(header.mode, Mode::BINARY);
}

#[test]
fn read_pgm_header_eight_sub_255() {
    let dat:Vec<u8> = vec![b'P', b'5', 10, b'6', b' ', b'1', b'0', 10, b'1', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 10);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::BINARY);
}

#[test]
fn read_pgm_header_sixteen_sub_65535() {
    let dat:Vec<u8> = vec![b'P', b'2', 10, b'6', b' ', b'1', b'0', 10, b'2', b'5', b'6', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 12);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::SIXTEEN);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_pgm_header_ascii_eight_commented() {
    let dat:Vec<u8> = vec![b'P', b'2', 10, b'6', b' ', b'1', b'0', 10, b'#', b'C', b'o', b'm', b'm', b'e', b'n', b't', 10, b'2', b'5', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 21);
    assert_eq!(header.image_type, ImageType::PGM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_ppm_header_ascii_eight() {
    let dat:Vec<u8> = vec![b'P', b'3', 10, b'6', b' ', b'1', b'0', 10, b'2', b'5', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 12);
    assert_eq!(header.image_type, ImageType::PPM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_ppm_header_binary_eight() {
    let dat:Vec<u8> = vec![b'P', b'6', 10, b'6', b' ', b'1', b'0', 10, b'2', b'5', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 12);
    assert_eq!(header.image_type, ImageType::PPM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::BINARY);
}

#[test]
fn read_ppm_header_ascii_sixteen() {
    let dat:Vec<u8> = vec![b'P', b'3', 10, b'6', b' ', b'1', b'0', 10, b'2', b'5', b'6', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 12);
    assert_eq!(header.image_type, ImageType::PPM);
    assert_eq!(header.depth, BitDepth::SIXTEEN);
    assert_eq!(header.mode, Mode::ASCII);
}

#[test]
fn read_ppm_header_binary_sixteen() {
    let dat:Vec<u8> = vec![b'P', b'6', 10, b'6', b' ', b'1', b'0', 10, b'1', b'0', b'5', b'0', b'0', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 14);
    assert_eq!(header.image_type, ImageType::PPM);
    assert_eq!(header.depth, BitDepth::SIXTEEN);
    assert_eq!(header.mode, Mode::BINARY);
}

#[test]
fn read_ppm_header_ascii_eight_commented() {
    let dat:Vec<u8> = vec![b'P', b'3', 10, b'6', b' ', b'1', b'0', 10, b'#', b'C', b'o', b'm', b'm', b'e', b'n', b't', 10, b'2', b'5', b'5', 10];
    let header = get_header(&dat).unwrap();
    assert_eq!(header.width, 6);
    assert_eq!(header.height, 10);
    assert_eq!(header.dat_start, 21);
    assert_eq!(header.image_type, ImageType::PPM);
    assert_eq!(header.depth, BitDepth::EIGHT);
    assert_eq!(header.mode, Mode::ASCII);
}
