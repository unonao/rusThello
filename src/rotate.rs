/*
    bitboardの回転用
*/

pub fn rotate180(x: u64) {
    let h1: u64 = 0x5555555555555555;
    let h2: u64 = 0x3333333333333333;
    let h4: u64 = 0x0F0F0F0F0F0F0F0F;
    let v1: u64 = 0x00FF00FF00FF00FF;
    let v2: u64 = 0x0000FFFF0000FFFF;
    let x = ((x >> 1) & h1) | ((x & h1) << 1);
    let x = ((x >> 2) & h2) | ((x & h2) << 2);
    let x = ((x >> 4) & h4) | ((x & h4) << 4);
    let x = ((x >> 8) & v1) | ((x & v1) << 8);
    let x = ((x >> 16) & v2) | ((x & v2) << 16);
    let x = (x >> 32) | (x << 32);
    return x;
}

pub fn flipVertical(x: u64) {
    let k1: u64 = (0x00FF00FF00FF00FF as u64);
    let k2: u64 = (0x0000FFFF0000FFFF as u64);
    let x = ((x >> 8) & k1) | ((x & k1) << 8);
    let x = ((x >> 16) & k2) | ((x & k2) << 16);
    let x = (x >> 32) | (x << 32);
    return x;
}

pub fn mirrorHorizontal(x: u64) {
    let k1: u64 = 0x5555555555555555;
    let k2: u64 = 0x3333333333333333;
    let k4: u64 = 0x0f0f0f0f0f0f0f0f;
    let x = ((x >> 1) & k1) | ((x & k1) << 1);
    let x = ((x >> 2) & k2) | ((x & k2) << 2);
    let x = ((x >> 4) & k4) | ((x & k4) << 4);
    return x;
}

pub fn rotate90clockwise(x: u64) {
    return flipVertical(flipDiagA1H8(x));
}
pub fn rotate90antiClockwise(x: u64) {
    return flipDiagA1H8(flipVertical(x));
}
