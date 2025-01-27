use core::fmt;

use crate::{u8_array_to_u16, u8_array_to_u64};

// In bytes
pub const EXIF_CHUNK_SIZE: usize = 20;

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
pub struct ExifChunk {
    tag: u16,
    data_type: u16,
    count: u64,
    value_offset: u64,
}

impl ExifChunk {
    pub fn from(slice: &[u8]) -> Self {
        Self {
            tag: match u8_array_to_u16(slice[0..2].to_vec()) {
                Some(converted_tag) => converted_tag,
                None => panic!("Invalid data tag"),
            },
            data_type: match u8_array_to_u16(slice[2..4].to_vec()) {
                Some(converted_data_type) => converted_data_type,
                None => panic!("Invalid data type"),
            },
            count: match u8_array_to_u64(slice[4..12].to_vec()) {
                Some(converted_count) => converted_count,
                None => panic!("Invalid count"),
            },
            value_offset: match u8_array_to_u64(slice[12..20].to_vec()) {
                Some(converted_value_offset) => converted_value_offset,
                None => panic!("Invalid value_offset"),
            },
        }
    }

    pub fn is_little_endian(&self) -> bool {
        false
    }
}

impl fmt::Display for ExifChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EXIF {{\n\ttag: {},\n\ttype: {},\n\tcount: {},\n\tvalue_offset: {}\n}}",
            self.tag, self.data_type, self.count, self.value_offset
        )
    }
}
