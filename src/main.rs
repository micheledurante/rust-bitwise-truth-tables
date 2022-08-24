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
    assert_eq!(0b00000000u8, and(0b00000000u8, 0b00000000u8));
    assert_eq!(0b00000000u8, and(0b00000000u8, 0b00000001u8));
    assert_eq!(0b00000000u8, and(0b00000001u8, 0b00000000u8));
    assert_eq!(0b00000001u8, and(0b00000001u8, 0b00000001u8));

    assert_eq!(0b00000000u8, or(0b00000000u8, 0b00000000u8));
    assert_eq!(0b00000001u8, or(0b00000000u8, 0b00000001u8));
    assert_eq!(0b00000001u8, or(0b00000001u8, 0b00000000u8));
    assert_eq!(0b00000001u8, or(0b00000001u8, 0b00000001u8));

    assert_eq!(0b00000000u8, xor(0b00000000u8, 0b00000000u8));
    assert_eq!(0b00000001u8, xor(0b00000000u8, 0b00000001u8));
    assert_eq!(0b00000001u8, xor(0b00000001u8, 0b00000000u8));
    assert_eq!(0b00000000u8, xor(0b00000001u8, 0b00000001u8));

    assert_eq!(0b11111111u8, not(0b00000000u8));
    assert_eq!(0b11111110u8, not(0b00000001u8));
}
