
pub fn decode(fl: u8) -> u32 {
    // get the significant digits (mantissa?)
    let n: u32 = fl as u32 & 0x0f;
    // get the exponent
    let exponent: u8 = (fl & 0xf0) >> 4;
    // calculate the result
    ((n as u32) << exponent) + (0xffff >> (16-exponent) << 4)
}

pub fn encode(value: u32) -> u8 {
    let mut overflow: u32 = 0;
    for exponent in 0..16 {
        if value < overflow * 2 + 16 {
            let mut fl = (value - overflow) >> exponent; // mantissa
            fl += exponent << 4; // exponent
            return fl as u8;
        }

        overflow = overflow * 2 + 16;
    }
    return 0; // unreachable for numbers in the range
}


#[test]
fn test() {
    let mut previous_value: i32 = -1;
    let mut success = true;
    for i in 0..256 as u32 {
        let fl: u8 = i as u8;
        let value: u32 = decode(fl);
        let fl2: u8 = encode(value);
        if fl != fl2 {
            println!("{:02x}: produces value {} but encodes back to {:02x}", fl, value, fl2);
            success = false;
        }
        if value as i32 <= previous_value {
            println!("{:02x}: value {} <= previous_value {}", fl, value, previous_value);
            success = false;
        }
        previous_value = value as i32;
    }
    assert!(success);
}
