use core::fmt;
use std::{env::args, fs};

/// Exif data of an image
///
/// tag: Each tag is assigned a unique 2-byte number to identify the field. The tag numbers in the Exif 0th
///      IFD and 1st IFD are all the same as the TIFF tag numbers.
///
/// data_type:  1 = BYTE An 8-bit unsigned integer.
///             2 = ASCII An 8-bit byte containing one 7-bit ASCII code.
///                 The final byte is terminated with NULL.
///             3 = SHORT A 16-bit (2-byte) unsigned integer,
///             4 = LONG A 32-bit (4-byte) unsigned integer,
///             5 = RATIONAL Two LONGs. The first LONG is the numerator and the second LONG
///                 expresses the denominator.
///             7 = UNDEFINED An 8-bit byte that may take any value depending on the field definition.
///             9 = SLONG A 32-bit (4-byte) signed integer (2's complement notation).
///             10 = SRATIONAL Two SLONGs. The first SLONG is the numerator and the second SLONG is
///                  the denominator.
///
/// count: The number of values. It should be noted carefully that the count is not the sum of the bytes. In the
///        case of one value of SHORT (16 bits), for example, the count is '1' even though it is 2 Bytes
///
/// value_offset: This tag records the offset from the start of the TIFF header to the position where the value itself is
///               recorded. In cases where the value fits in 4 Bytes, the value itself is recorded. If the value is smaller
///               than 4 Bytes, the value is stored in the 4-Byte area starting from the left, i.e., from the lower end of
///               the byte offset area. For example, in big endian format, if the type is SHORT and the value is 1, it is
///               recorded as 00010000.H.
///               Note that field Interoperability shall be recorded in sequence starting from the smallest tag number.
///               There is no stipulation regarding the order or position of tag value (Value) recording.
///
struct ExifData {
    tag: u16,
    data_type: u16,
    count: u64,
    value_offset: u64,
}

impl ExifData {
    pub fn from(tag: &[u8], data_type: &[u8], count: &[u8], value_offset: &[u8]) -> Self {
        Self {
            tag: match u8_array_to_u16(tag.to_vec()) {
                Some(converted_tag) => converted_tag,
                None => panic!("invalid data tag"),
            },
            data_type: match u8_array_to_u16(data_type.to_vec()) {
                Some(converted_data_type) => converted_data_type,
                None => panic!("invalid data type"),
            },
            count: match u8_array_to_u64(count.to_vec()) {
                Some(converted_count) => converted_count,
                None => panic!("invalid count"),
            },
            value_offset: match u8_array_to_u64(value_offset.to_vec()) {
                Some(converted_value_offset) => converted_value_offset,
                None => panic!("invalid value_offset"),
            },
        }
    }
}

impl fmt::Display for ExifData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(tag: {}, type: {}, count: {}, value_offset: {})",
            self.tag, self.data_type, self.count, self.value_offset
        )
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return;
    }

    let img_path = match args.get(1) {
        Some(img_path) => img_path,
        None => return,
    };

    let img_contents = match fs::read(img_path) {
        Ok(img_contents) => img_contents,
        Err(e) => return eprintln!("{e:?}"),
    };

    let exif_chars = vec![101, 88, 73, 102]; //eXIf
    let exif_start_idx = index_of_sub_array(img_contents.clone(), exif_chars).unwrap() + 4;
    println!("{}", *img_contents.get(exif_start_idx + 1).unwrap() as char);

    /*println!("{}", *img_contents.get(256).unwrap());

    println!(
        "{}",
        u8_array_to_u16(img_contents[255..257].to_vec()).unwrap()
    );

    println!(
        "{}",
        u8_array_to_u32(img_contents[255..259].to_vec()).unwrap()
    );*/

    let block = img_contents[exif_start_idx + 1..exif_start_idx + 23].to_vec();
    let exif_data = ExifData::from(
        block[0..2].as_ref(),
        block[2..4].as_ref(),
        block[4..12].as_ref(),
        block[12..20].as_ref(),
    );
    println!("{}", exif_data);
}

fn u8_array_to_u16(u8_array: Vec<u8>) -> Option<u16> {
    if u8_array.len() != 2 {
        return None;
    }

    Some(((u8_array[0] as u16) << 8) | (u8_array[1] as u16))
}

fn u8_array_to_u64(u8_array: Vec<u8>) -> Option<u64> {
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

fn index_of_sub_array(a: Vec<u8>, sub_a: Vec<u8>) -> Option<usize> {
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
