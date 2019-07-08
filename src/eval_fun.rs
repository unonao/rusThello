use crate::rotate::*;
use std::fs;
use std::fs::File;
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
    pub static ref MODEL: Vec<Index> = {
        let mut model = Vec::new();
        for stage in 0..15 {
            let content = fs::read_to_string(format!("./model/stage{}.txt", stage)).unwrap();
            let deserialized: Index = serde_json::from_str(&content).unwrap();
            model.push(deserialized);
        }
        model
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub diag4: Vec<f32>,
    pub diag5: Vec<f32>,
    pub diag6: Vec<f32>,
    pub diag7: Vec<f32>,
    pub diag8: Vec<f32>,
    pub hv2: Vec<f32>,
    pub hv3: Vec<f32>,
    pub hv4: Vec<f32>,
    pub edge2x: Vec<f32>,
    pub cor25h: Vec<f32>,
    pub cor25v: Vec<f32>,
    pub cor33: Vec<f32>,
    pub def: f32,
    pub next_mobility_num: f32,
}

pub fn eval_by_model(
    next_ori: &u64,
    pre_ori: &u64,
    def: &f32,
    next_mobility_num: &f32,
    mut index: &mut Index,
) -> f32 {
    let mut result = 0.0;

    let next = next_ori;
    let pre = pre_ori;

    result += index.diag4[board_to_diag4(&next, &pre) as usize]
        + index.diag5[board_to_diag5(&next, &pre) as usize]
        + index.diag6[board_to_diag6(&next, &pre) as usize]
        + index.diag7[board_to_diag7(&next, &pre) as usize]
        + index.diag8[board_to_diag8(&next, &pre) as usize]
        + index.hv2[board_to_hv2(&next, &pre) as usize]
        + index.hv3[board_to_hv3(&next, &pre) as usize]
        + index.hv4[board_to_hv4(&next, &pre) as usize]
        + index.edge2x[board_to_edge2x(&next, &pre) as usize]
        + index.cor25h[board_to_cor25h(&next, &pre) as usize]
        + index.cor25v[board_to_cor25v(&next, &pre) as usize]
        + index.cor33[board_to_cor33(&next, &pre) as usize];

    let next: u64 = flip_diag_a1h8(next_ori);
    let pre: u64 = flip_diag_a1h8(pre_ori);

    result += index.diag4[board_to_diag4(&next, &pre) as usize]
        + index.diag5[board_to_diag5(&next, &pre) as usize]
        + index.diag6[board_to_diag6(&next, &pre) as usize]
        + index.diag7[board_to_diag7(&next, &pre) as usize]
        //+ index.diag8[board_to_diag8(&next, &pre) as usize]
        + index.hv2[board_to_hv2(&next, &pre) as usize]
        + index.hv3[board_to_hv3(&next, &pre) as usize]
        + index.hv4[board_to_hv4(&next, &pre) as usize]
        + index.edge2x[board_to_edge2x(&next, &pre) as usize]
        + index.cor25h[board_to_cor25h(&next, &pre) as usize]
        + index.cor25v[board_to_cor25v(&next, &pre) as usize]
        + index.cor33[board_to_cor33(&next, &pre) as usize];
    let next: u64 = rotate180(&flip_diag_a1h8(next_ori));
    let pre: u64 = rotate180(&flip_diag_a1h8(pre_ori));

    result += index.diag4[board_to_diag4(&next, &pre) as usize]
        + index.diag5[board_to_diag5(&next, &pre) as usize]
        + index.diag6[board_to_diag6(&next, &pre) as usize]
        + index.diag7[board_to_diag7(&next, &pre) as usize]
        //+ index.diag8[board_to_diag8(&next, &pre) as usize]
        + index.hv2[board_to_hv2(&next, &pre) as usize]
        + index.hv3[board_to_hv3(&next, &pre) as usize]
        + index.hv4[board_to_hv4(&next, &pre) as usize]
        + index.edge2x[board_to_edge2x(&next, &pre) as usize]
        + index.cor25h[board_to_cor25h(&next, &pre) as usize]
        + index.cor25v[board_to_cor25v(&next, &pre) as usize]
        + index.cor33[board_to_cor33(&next, &pre) as usize];

    let next: u64 = rotate180(next_ori);
    let pre: u64 = rotate180(pre_ori);
    result += index.diag4[board_to_diag4(&next, &pre) as usize]
        + index.diag5[board_to_diag5(&next, &pre) as usize]
        + index.diag6[board_to_diag6(&next, &pre) as usize]
        + index.diag7[board_to_diag7(&next, &pre) as usize]
        + index.diag8[board_to_diag8(&next, &pre) as usize]
        + index.hv2[board_to_hv2(&next, &pre) as usize]
        + index.hv3[board_to_hv3(&next, &pre) as usize]
        + index.hv4[board_to_hv4(&next, &pre) as usize]
        + index.edge2x[board_to_edge2x(&next, &pre) as usize]
        + index.cor25h[board_to_cor25h(&next, &pre) as usize]
        + index.cor25v[board_to_cor25v(&next, &pre) as usize]
        + index.cor33[board_to_cor33(&next, &pre) as usize];

    result += index.def * def + index.next_mobility_num * next_mobility_num;
    result as f32
}

pub fn board_to_diag4(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^4-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_diag5(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^5-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_diag6(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^6-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_diag7(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^7-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_diag8(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}

pub fn board_to_hv2(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_hv3(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_hv4(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_edge2x(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_cor25h(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_cor25v(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
pub fn board_to_cor33(next: &u64, pre: &u64) -> usize {
    // 0 ~ 3^8-1
    let mask: Vec<u64> = vec![
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
    result as usize
}
