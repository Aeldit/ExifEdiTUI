use core::fmt;

use crate::{
    u8_2_to_u16_be, u8_2_to_u16_le, u8_4_to_u32_be, u8_4_to_u32_le, u8_8_to_u64_be, u8_8_to_u64_le,
};

// In bytes
pub const TIFF_HEADER_SIZE: usize = 20;
pub const EXIF_CHUNK_SIZE: usize = 20;

pub enum MagicBytes {
    Png(),
    Jpeg(),
}

pub struct TIFFHeader {
    byte_order: (u8, u8),
    fixed: (u8, u8),
    ifd_offset: (u8, u8, u8, u8), // If = 8 => followed directly by the 0th IFD
}

pub struct Ifd {
    pub number_of_fields: (u8, u8),
    pub interoperability_arrays: InteroperabilityField,
} // 4 byte offset to the next IFD

/// Exif chunk of an image
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
pub struct InteroperabilityField {
    tag: (u8, u8),
    data_type: (u8, u8),
    count: (u8, u8, u8, u8, u8, u8, u8, u8),
    value_offset: (u8, u8, u8, u8, u8, u8, u8, u8),
    // Not in the chunk
    is_little_endian: bool,
}

impl TIFFHeader {
    pub fn from(slice: &[u8]) -> Self {
        if slice.len() != TIFF_HEADER_SIZE {
            panic!(
                "Invalid slice length for the TIFFHeader (expected {} but got {})",
                TIFF_HEADER_SIZE,
                slice.len(),
            );
        }

        Self {
            byte_order: (slice[0], slice[1]),
            fixed: (slice[2], slice[3]),
            ifd_offset: (slice[4], slice[5], slice[6], slice[7]),
        }
    }

    pub fn is_little_endian(&self) -> bool {
        self.byte_order == (0x49, 0x49)
    }

    pub fn get_0th_idf_offset(&self) -> u32 {
        let off = if self.is_little_endian() {
            u8_4_to_u32_le(self.ifd_offset)
        } else {
            u8_4_to_u32_be(self.ifd_offset)
        };

        if off == 8 {
            0
        } else {
            off
        }
    }
}

impl fmt::Display for TIFFHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TIFF {{\n\tbyte order: {} {},\n\tfixed: {} {},\n\tifd offset: {} {} {} {} => {}\n}}",
            self.byte_order.0,
            self.byte_order.1,
            self.fixed.0,
            self.fixed.1,
            self.ifd_offset.0,
            self.ifd_offset.1,
            self.ifd_offset.2,
            self.ifd_offset.3,
            if self.is_little_endian() {
                u8_4_to_u32_le(self.ifd_offset)
            } else {
                u8_4_to_u32_be(self.ifd_offset)
            },
        )
    }
}

impl fmt::Display for Ifd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IFD {{\n\tnumber of fields: {} {},\n\tinteroperability: {}\n}}",
            self.number_of_fields.0, self.number_of_fields.1, self.interoperability_arrays,
        )
    }
}

impl InteroperabilityField {
    pub fn from(slice: &[u8], is_little_endian: bool) -> Self {
        if slice.len() != EXIF_CHUNK_SIZE {
            panic!(
                "Invalid len for the Exif slice (expected {} but got {})",
                EXIF_CHUNK_SIZE,
                slice.len(),
            );
        }

        Self {
            tag: (slice[0], slice[1]),
            data_type: (slice[2], slice[3]),
            count: (
                slice[4], slice[5], slice[6], slice[7], slice[8], slice[9], slice[10], slice[11],
            ),
            value_offset: (
                slice[12], slice[13], slice[14], slice[15], slice[16], slice[17], slice[18],
                slice[19],
            ),
            is_little_endian,
        }
    }
}

impl fmt::Display for InteroperabilityField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {{
            tag: {} {} => {},
            type: {} {} => {},
            count: {} {} {} {} {} {} {} {} => {},
            value_offset: {} {} {} {} {} {} {} {} => {}\n\t}}",
            if self.is_little_endian { "LE" } else { "BE" },
            self.tag.0,
            self.tag.1,
            if self.is_little_endian {
                u8_2_to_u16_le(self.tag)
            } else {
                u8_2_to_u16_be(self.tag)
            },
            self.data_type.0,
            self.data_type.1,
            if self.is_little_endian {
                u8_2_to_u16_le(self.data_type)
            } else {
                u8_2_to_u16_be(self.data_type)
            },
            self.count.0,
            self.count.1,
            self.count.2,
            self.count.3,
            self.count.4,
            self.count.5,
            self.count.6,
            self.count.7,
            if self.is_little_endian {
                u8_8_to_u64_le(self.count)
            } else {
                u8_8_to_u64_be(self.count)
            },
            self.value_offset.0,
            self.value_offset.1,
            self.value_offset.2,
            self.value_offset.3,
            self.value_offset.4,
            self.value_offset.5,
            self.value_offset.6,
            self.value_offset.7,
            if self.is_little_endian {
                u8_8_to_u64_le(self.value_offset)
            } else {
                u8_8_to_u64_be(self.value_offset)
            },
        )
    }
}
