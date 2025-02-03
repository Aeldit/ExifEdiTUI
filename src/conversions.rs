pub fn index_of_sub_array(a: Vec<u8>, sub_a: Vec<u8>) -> Option<usize> {
    let mut i = 0;
    let len = sub_a.len();
    for (a_idx, b) in a.into_iter().enumerate() {
        if b == *sub_a.get(i).unwrap() {
            i += 1;
        } else {
            i = 0;
        }

        if i == len {
            return Some(a_idx - 3);
        }
    }

    None
}

pub fn u8_2_to_u16_be(u: (u8, u8)) -> u16 {
    ((u.0 as u16) << 8) | (u.1 as u16)
}

pub fn u8_2_to_u16_le(u: (u8, u8)) -> u16 {
    ((u.1 as u16) << 8) | (u.0 as u16)
}

pub fn u8_4_to_u32_be(u: (u8, u8, u8, u8)) -> u32 {
    ((u.0 as u32) << 24) | ((u.1 as u32) << 16) | ((u.2 as u32) << 8) | (u.3 as u32)
}

pub fn u8_4_to_u32_le(u: (u8, u8, u8, u8)) -> u32 {
    ((u.3 as u32) << 24) | ((u.2 as u32) << 16) | ((u.1 as u32) << 8) | (u.0 as u32)
}

pub fn u8_4_to_i32_le(u: (u8, u8, u8, u8)) -> i32 {
    ((u.3 as i32) << 24) | ((u.2 as i32) << 16) | ((u.1 as i32) << 8) | (u.0 as i32)
}
