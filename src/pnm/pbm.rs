/// Load image data stored in ASCII format.
pub fn load_ascii(inpt: &[u8]) -> Vec<u8> {
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
pub fn load_binary(inpt: &[u8], width: u32) -> Vec<u8> {
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
