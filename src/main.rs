fn not(a: u8) -> u8 {
    !a
}

fn xor(a: u8, b: u8) -> u8 {
    a ^ b
}

fn or(a: u8, b: u8) -> u8 {
    a | b
}

fn and(a: u8, b: u8) -> u8 {
    a & b
}

fn main() {
    assert_eq!(0b0000_0000u8, and(0b0000_0000u8, 0b0000_0000u8));
    assert_eq!(0b0000_0000u8, and(0b0000_0000u8, 0b0000_0001u8));
    assert_eq!(0b0000_0000u8, and(0b0000_0001u8, 0b0000_0000u8));
    assert_eq!(0b0000_0001u8, and(0b0000_0001u8, 0b0000_0001u8));

    assert_eq!(0b0000_0000u8, or(0b0000_0000u8, 0b0000_0000u8));
    assert_eq!(0b0000_0001u8, or(0b0000_0000u8, 0b0000_0001u8));
    assert_eq!(0b0000_0001u8, or(0b0000_0001u8, 0b0000_0000u8));
    assert_eq!(0b0000_0001u8, or(0b0000_0001u8, 0b0000_0001u8));

    assert_eq!(0b0000_0000u8, xor(0b0000_0000u8, 0b0000_0000u8));
    assert_eq!(0b0000_0001u8, xor(0b0000_0000u8, 0b0000_0001u8));
    assert_eq!(0b0000_0001u8, xor(0b0000_0001u8, 0b0000_0000u8));
    assert_eq!(0b0000_0000u8, xor(0b0000_0001u8, 0b0000_0001u8));

    assert_eq!(0b1111_1111u8, not(0b0000_0000u8));
    assert_eq!(0b1111_1110u8, not(0b0000_0001u8));
}
