/*
    bitboardの回転用
*/

pub fn rotate180(x: &u64) -> u64 {
    let h1: u64 = 0x5555555555555555;
    let h2: u64 = 0x3333333333333333;
    let h4: u64 = 0x0F0F0F0F0F0F0F0F;
    let v1: u64 = 0x00FF00FF00FF00FF;
    let v2: u64 = 0x0000FFFF0000FFFF;
    let x: u64 = ((x >> 1) & h1) | ((x & h1) << 1);
    let x: u64 = ((x >> 2) & h2) | ((x & h2) << 2);
    let x: u64 = ((x >> 4) & h4) | ((x & h4) << 4);
    let x: u64 = ((x >> 8) & v1) | ((x & v1) << 8);
    let x: u64 = ((x >> 16) & v2) | ((x & v2) << 16);
    let x: u64 = (x >> 32) | (x << 32);
    return x;
}

pub fn flip_vertical(x: &u64) -> u64 {
    let k1: u64 = 0x00FF00FF00FF00FF as u64;
    let k2: u64 = 0x0000FFFF0000FFFF as u64;
    let x: u64 = ((x >> 8) & k1) | ((x & k1) << 8);
    let x: u64 = ((x >> 16) & k2) | ((x & k2) << 16);
    let x: u64 = (x >> 32) | (x << 32);
    return x;
}

pub fn mirror_horizontal(x: &u64) -> u64 {
    let k1: u64 = 0x5555555555555555;
    let k2: u64 = 0x3333333333333333;
    let k4: u64 = 0x0f0f0f0f0f0f0f0f;
    let x = ((x >> 1) & k1) | ((x & k1) << 1);
    let x = ((x >> 2) & k2) | ((x & k2) << 2);
    let x = ((x >> 4) & k4) | ((x & k4) << 4);
    return x;
}

pub fn flip_diag_a1h8(x: &u64) -> u64 {
    let k1: u64 = 0x5500550055005500;
    let k2: u64 = 0x3333000033330000;
    let k4: u64 = 0x0f0f0f0f00000000;
    let t = k4 & (x ^ (x << 28));
    let mut x = x ^ t ^ (t >> 28);
    let t = k2 & (x ^ (x << 14));
    x ^= t ^ (t >> 14);
    let t = k1 & (x ^ (x << 7));
    x ^= t ^ (t >> 7);
    return x;
}

pub fn rotate90_anti_clockwise(x: &u64) -> u64 {
    return flip_vertical(&flip_diag_a1h8(x));
}
pub fn rotate90_clockwise(x: &u64) -> u64 {
    return flip_diag_a1h8(&flip_vertical(x));
}
