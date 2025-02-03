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
