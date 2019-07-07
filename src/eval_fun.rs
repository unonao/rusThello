use crate::color::*;

lazy_static! {
    pub static ref THREE: Vec<i32> = vec![
        1,
        3,
        3 * 3,
        3 * 3 * 3,
        3 * 3 * 3 * 3,
        3 * 3 * 3 * 3 * 3,
        3 * 3 * 3 * 3 * 3 * 3,
        3 * 3 * 3 * 3 * 3 * 3 * 3,
        3 * 3 * 3 * 3 * 3 * 3 * 3 * 3,
        3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3,
    ];
}

pub fn board_to_diag4(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^4-1
    let mut mask: Vec<u64> = vec![
        0x1000000000000000,
        0x0020000000000000,
        0x0000400000000000,
        0x0000008000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_diag5(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^5-1
    let mut mask: Vec<u64> = vec![
        0x0800000000000000,
        0x0010000000000000,
        0x0000200000000000,
        0x0000004000000000,
        0x0000000080000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_diag6(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^6-1
    let mut mask: Vec<u64> = vec![
        0x0400000000000000,
        0x0008000000000000,
        0x0000100000000000,
        0x0000002000000000,
        0x0000000040000000,
        0x0000000000800000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_diag7(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^7-1
    let mut mask: Vec<u64> = vec![
        0x0200000000000000,
        0x0004000000000000,
        0x0000080000000000,
        0x0000001000000000,
        0x0000000020000000,
        0x0000000000400000,
        0x0000000000008000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_diag8(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x0100000000000000,
        0x0002000000000000,
        0x0000040000000000,
        0x0000000800000000,
        0x0000000010000000,
        0x0000000000200000,
        0x0000000000004000,
        0x0000000000000080,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}

pub fn board_to_hv2(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x0001000000000000,
        0x0002000000000000,
        0x0004000000000000,
        0x0008000000000000,
        0x0010000000000000,
        0x0020000000000000,
        0x0040000000000000,
        0x0080000000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_hv3(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x0000010000000000,
        0x0000020000000000,
        0x0000040000000000,
        0x0000080000000000,
        0x0000100000000000,
        0x0000200000000000,
        0x0000400000000000,
        0x0000800000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_hv4(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x0000000100000000,
        0x0000000200000000,
        0x0000000400000000,
        0x0000000800000000,
        0x0000001000000000,
        0x0000002000000000,
        0x0000004000000000,
        0x0000008000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_edge2x(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x01000000000000,
        0x02000000000000,
        0x04000000000000,
        0x08000000000000,
        0x10000000000000,
        0x20000000000000,
        0x40000000000000,
        0x80000000000000,
        0x00400000000000,
        0x00020000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_cor25h(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x08000000000000,
        0x10000000000000,
        0x20000000000000,
        0x40000000000000,
        0x80000000000000,
        0x00080000000000,
        0x00100000000000,
        0x00200000000000,
        0x00400000000000,
        0x00800000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_cor25h(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x08000000000000,
        0x10000000000000,
        0x20000000000000,
        0x40000000000000,
        0x80000000000000,
        0x00080000000000,
        0x00100000000000,
        0x00200000000000,
        0x00400000000000,
        0x00800000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_cor25v(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x40000000000000,
        0x80000000000000,
        0x00400000000000,
        0x00800000000000,
        0x00004000000000,
        0x00008000000000,
        0x00000040000000,
        0x00000080000000,
        0x00000000400000,
        0x00000000800000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
pub fn board_to_cor33(next: u64, pre: u64) -> i32 {
    // 0 ~ 3^8-1
    let mut mask: Vec<u64> = vec![
        0x20000000000000,
        0x40000000000000,
        0x80000000000000,
        0x00200000000000,
        0x00400000000000,
        0x00800000000000,
        0x00002000000000,
        0x00004000000000,
        0x00008000000000,
    ];
    let mut result: i32 = 0;
    let mut count = 0;
    for i in mask {
        if (next & i) == i {
            result += 2 * THREE[count];
        } else if (pre & i) == i {
            result += THREE[count];
        }
        count += 1;
    }
    result
}
