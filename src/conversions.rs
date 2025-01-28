pub fn u8_array_to_u16_le(u8_array: Vec<u8>) -> Option<u16> {
    if u8_array.len() != 2 {
        return None;
    }

    Some(((u8_array[0] as u16) << 8) | (u8_array[1] as u16))
}

pub fn u8_array_to_u32_le(u8_array: Vec<u8>) -> Option<u32> {
    if u8_array.len() != 4 {
        return None;
    }

    Some(
        ((u8_array[0] as u32) << 24)
            | ((u8_array[1] as u32) << 16)
            | ((u8_array[2] as u32) << 8)
            | (u8_array[3] as u32),
    )
}

pub fn u8_array_to_u64_le(u8_array: Vec<u8>) -> Option<u64> {
    if u8_array.len() != 8 {
        return None;
    }

    Some(
        ((u8_array[0] as u64) << 56)
            | ((u8_array[1] as u64) << 48)
            | ((u8_array[2] as u64) << 40)
            | ((u8_array[3] as u64) << 32)
            | ((u8_array[4] as u64) << 24)
            | ((u8_array[5] as u64) << 16)
            | ((u8_array[6] as u64) << 8)
            | (u8_array[7] as u64),
    )
}

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

pub fn u8_8_to_u64_be(u: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u64 {
    ((u.0 as u64) << 56)
        | ((u.1 as u64) << 48)
        | ((u.2 as u64) << 40)
        | ((u.3 as u64) << 32)
        | ((u.4 as u64) << 24)
        | ((u.5 as u64) << 16)
        | ((u.6 as u64) << 8)
        | (u.7 as u64)
}

pub fn u8_8_to_u64_le(u: (u8, u8, u8, u8, u8, u8, u8, u8)) -> u64 {
    ((u.7 as u64) << 56)
        | ((u.6 as u64) << 48)
        | ((u.5 as u64) << 40)
        | ((u.4 as u64) << 32)
        | ((u.3 as u64) << 24)
        | ((u.2 as u64) << 16)
        | ((u.1 as u64) << 8)
        | (u.0 as u64)
}
