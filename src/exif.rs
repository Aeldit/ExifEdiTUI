use core::fmt;

use crate::{
    u8_2_to_u16_be, u8_2_to_u16_le, u8_4_to_u32_be, u8_4_to_u32_le, u8_8_to_u64_be, u8_8_to_u64_le,
};

// In bytes
pub const TIFF_HEADER_SIZE: usize = 8;
pub const INTEROPERABILITY_FIELD_SIZE: usize = 12;

pub enum MagicBytes {
    Png(),
    Jpeg(),
}

pub enum ExifTypes {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    Undefined,
    Slong,
    Srational,
    // Not defined by the spec
    Error,
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

pub struct InteroperabilityField {
    tag: (u8, u8),
    data_type: (u8, u8),
    count: (u8, u8, u8, u8),
    value_offset: (u8, u8, u8, u8),
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
            "TIFF {{ byte order: {} {}, fixed: {} {}, ifd offset: {} {} {} {} => {} }}",
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
        if slice.len() != INTEROPERABILITY_FIELD_SIZE {
            panic!(
                "Invalid len for the Exif slice (expected {} but got {})",
                INTEROPERABILITY_FIELD_SIZE,
                slice.len(),
            );
        }

        Self {
            tag: (slice[0], slice[1]),
            data_type: (slice[2], slice[3]),
            count: (slice[4], slice[5], slice[6], slice[7]),
            value_offset: (slice[8], slice[9], slice[10], slice[11]),
            is_little_endian,
        }
    }

    pub fn get_type(&self) -> ExifTypes {
        match if self.is_little_endian {
            u8_2_to_u16_le(self.data_type)
        } else {
            u8_2_to_u16_be(self.data_type)
        } {
            1 => ExifTypes::Byte,
            2 => ExifTypes::Ascii,
            3 => ExifTypes::Short,
            4 => ExifTypes::Long,
            5 => ExifTypes::Rational,
            7 => ExifTypes::Undefined,
            9 => ExifTypes::Slong,
            10 => ExifTypes::Srational,
            _ => ExifTypes::Error,
        }
    }

    pub fn get_count_as_string(&self) -> String {
        if self.is_little_endian {
            // TODO: Use the value instead of the count
            let count = u8_4_to_u32_le(self.count);
            match self.get_type() {
                ExifTypes::Byte => todo!(),
                ExifTypes::Ascii => todo!(),
                ExifTypes::Short => todo!(),
                ExifTypes::Long => todo!(),
                ExifTypes::Rational => {
                    let numerator = (count >> 16) as u16;
                    let denominator = count as u16;
                    format!("{} / {}", numerator, denominator)
                }
                ExifTypes::Undefined => todo!(),
                ExifTypes::Slong => todo!(),
                ExifTypes::Srational => todo!(),
                ExifTypes::Error => todo!(),
            }
        } else {
            String::new()
        }
    }
}

impl fmt::Display for InteroperabilityField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {{
            tag: {} {} => {},
            type: {} {} => {} ({}),
            count: {} {} {} {} => {},
            value_offset: {} {} {} {} => {}\n\t}}",
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
            get_type_as_string(self.get_type()),
            self.count.0,
            self.count.1,
            self.count.2,
            self.count.3,
            if self.is_little_endian {
                u8_4_to_u32_le(self.count)
            } else {
                u8_4_to_u32_be(self.count)
            },
            self.value_offset.0,
            self.value_offset.1,
            self.value_offset.2,
            self.value_offset.3,
            if self.is_little_endian {
                u8_4_to_u32_le(self.value_offset)
            } else {
                u8_4_to_u32_be(self.value_offset)
            },
        )
    }
}

fn get_type_as_string(exif_type: ExifTypes) -> String {
    match exif_type {
        ExifTypes::Byte => String::from("Byte"),
        ExifTypes::Ascii => String::from("Ascii"),
        ExifTypes::Short => String::from("Short"),
        ExifTypes::Long => String::from("Long"),
        ExifTypes::Rational => String::from("Rational"),
        ExifTypes::Undefined => String::from("Undefined"),
        ExifTypes::Slong => String::from("Slong"),
        ExifTypes::Srational => String::from("Srational"),
        ExifTypes::Error => String::from("Error"),
    }
}
