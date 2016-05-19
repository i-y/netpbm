extern crate netbpm;
use netbpm::pgm::PGMEncoder;
use netbpm::{Mode,BitDepth};

// 6 10
const J:[u8;60] = [255,255,255,255,0,255,
                   255,255,255,255,0,255,
                   255,255,255,255,0,255,
                   255,255,255,255,0,255,
                   255,255,255,255,0,255,
                   255,255,255,255,0,255,
                   0,  255,255,255,0,255,
                   255,0,0,0,255,255,
                   255,255,255,255,255,255,
                   255,255,255,255,255,255];

// 6 10
const J_DOUBLE:[u8;120] = [255,255 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           255,255 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           255,255 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           255,255 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           255,255 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           255,255 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           0,0 ,255,255 ,255,255 ,255,255 ,0,0 ,255,255,
                           255,255 ,0,0 ,0,0 ,0,0 ,255,255 ,255,255,
                           255,255,255,255,255,255,255,255,255,255,255,255,
                           255,255,255,255,255,255,255,255,255,255,255,255];



#[test]
fn pgm_j_single_ascii() {
    let mut encoder = PGMEncoder::new("test_0a.pgm");
    let enc_result = encoder.save(&J, 6, 10, Mode::ASCII, BitDepth::EIGHT);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn pgm_j_single_binary() {
    let mut encoder = PGMEncoder::new("test_0b.pgm");
    let enc_result = encoder.save(&J, 6, 10, Mode::BINARY, BitDepth::EIGHT);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn pgm_j_double_ascii() {
    let mut encoder = PGMEncoder::new("test_1a.pgm");
    let enc_result = encoder.save(&J_DOUBLE, 6, 10, Mode::ASCII, BitDepth::SIXTEEN);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn pgm_j_double_binary() {
    let mut encoder = PGMEncoder::new("test_1b.pgm");
    let enc_result = encoder.save(&J_DOUBLE, 6, 10, Mode::BINARY, BitDepth::SIXTEEN);
    match enc_result {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}
