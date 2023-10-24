pub fn get_bits(number: u32, lsb: u8, msb: u8) -> u32 {
    assert!(lsb <= msb);

    let nb_bits = msb - lsb + 1;
    let mask = (1 << nb_bits) - 1;
    let number = number >> lsb;

    number & mask
}

pub fn sign_extend_number(number: u32, nb_bits: u32) -> u32 {
    assert!(nb_bits < 32);

    let bit = (nb_bits - 1) as u8;
    let is_signed = get_bits(number, bit, bit) == 1;
    let mut number = number;
    if is_signed {
        for i in nb_bits..32 {
            number |= 1 << i;
        }
    }
    number
}