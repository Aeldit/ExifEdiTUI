use std::fmt;

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

pub fn get_vec_as_string<T: fmt::Display>(vec: Vec<T>) -> String {
    let mut s = String::new();
    let len = vec.len();
    for (i, value) in vec.iter().enumerate() {
        if i == len - 1 {
            s.push_str(format!("{}", value).as_str());
        } else {
            s.push_str(format!("{} ", value).as_str());
        }
    }

    s
}

pub fn get_ratioanls_vec_as_string<T: fmt::Display>(vec: Vec<(T, T)>) -> String {
    let mut s = String::new();
    let len = vec.len();
    for (i, value) in vec.iter().enumerate() {
        if i == len - 1 {
            s.push_str(format!("{}/{}", value.0, value.1).as_str());
        } else {
            s.push_str(format!("{}/{} ", value.0, value.1).as_str());
        }
    }

    s
}
