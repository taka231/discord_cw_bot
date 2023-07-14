/*
    returns (length, binary expression)
    0 = dot, 1 = dash; MSB first
*/
pub fn get_morse(c: char) -> (u8, u8) {
    let c = c.to_ascii_uppercase();
    match c {
        '0' => (5, 0b11111),
        '1' => (5, 0b01111),
        '2' => (5, 0b00111),
        '3' => (5, 0b00011),
        '4' => (5, 0b00001),
        '5' => (5, 0b00000),
        '6' => (5, 0b10000),
        '7' => (5, 0b11000),
        '8' => (5, 0b11100),
        '9' => (5, 0b11110),
        'A' => (2, 0b01),
        'B' => (4, 0b1000),
        'C' => (4, 0b1010),
        'D' => (3, 0b100),
        'E' => (1, 0b0),
        'F' => (4, 0b0010),
        'G' => (3, 0b110),
        'H' => (4, 0b0000),
        'I' => (2, 0b00),
        'J' => (4, 0b0111),
        'K' => (3, 0b101),
        'L' => (4, 0b0100),
        'M' => (2, 0b11),
        'N' => (2, 0b10),
        'O' => (3, 0b111),
        'P' => (4, 0b0110),
        'Q' => (4, 0b1101),
        'R' => (3, 0b010),
        'S' => (3, 0b000),
        'T' => (1, 0b1),
        'U' => (3, 0b001),
        'V' => (4, 0b0001),
        'W' => (3, 0b011),
        'X' => (4, 0b1001),
        'Y' => (4, 0b1011),
        'Z' => (4, 0b1100),
        '!' => (6, 0b101011),
        '"' => (6, 0b010010),
        '$' => (7, 0b0001001),
        '&' => (5, 0b01000),
        '(' => (5, 0b10110),
        ')' => (6, 0b101101),
        '+' => (5, 0b01010),
        ',' => (6, 0b110011),
        '-' => (6, 0b100001),
        '.' => (6, 0b010101),
        '/' => (5, 0b10010),
        ':' => (6, 0b111000),
        ';' => (6, 0b101010),
        '=' => (5, 0b10001),
        '?' => (6, 0b001100),
        '@' => (6, 0b011010),
        '\'' => (6, 0b011110),
        '_' => (6, 0b001101),
        _ => (0, 0),
    }
}

pub fn dot_time(wpm: f32) -> std::time::Duration {
    std::time::Duration::from_secs_f32(1.2 / wpm)
}
