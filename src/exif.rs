use core::fmt;

use crate::{u8_2_to_u16_be, u8_2_to_u16_le, u8_4_to_u32_be, u8_4_to_u32_le};

// In bytes
pub const TIFF_HEADER_SIZE: usize = 8;
pub const INTEROPERABILITY_FIELD_SIZE: usize = 12;

/*pub enum MagicBytes {
    Png(),
    Jpeg(),
}*/

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

    pub fn get_as_string(&self) -> String {
        format!(
            "TIFF {{ Byte Order: {}, 0th IFD offset: {} }}",
            if self.is_little_endian() {
                "Little endian (II)"
            } else {
                "Big endian (MM)"
            },
            {
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
        )
    }
}

pub struct Ifd {
    pub number_of_fields: (u8, u8),
    pub interoperability_arrays: Vec<InteroperabilityField>, // Vec of size number_of_fields
    // Not in teh spec
    is_little_endian: bool,
} // 4 byte offset to the next IFD

impl Ifd {
    pub fn from(slice: &[u8], is_little_endian: bool) -> Self {
        if slice.len() < 2 {
            panic!(
                "Expected the slice length to be at least 2 for the number_of_fields, but got {}",
                slice.len()
            );
        }

        let number_of_fields = if is_little_endian {
            u8_2_to_u16_le((slice[0], slice[1]))
        } else {
            u8_2_to_u16_be((slice[0], slice[1]))
        };

        if slice.len() < 2 + INTEROPERABILITY_FIELD_SIZE * number_of_fields as usize {
            panic!("Expected the slice length to be at least {} with the interoperability fields, but got {}",
                2 + INTEROPERABILITY_FIELD_SIZE * number_of_fields as usize,
                slice.len()
            )
        }

        let mut interoperatibility_array = Vec::with_capacity(number_of_fields as usize);
        let mut chunk_start_idx = 2;
        for _ in 0..number_of_fields {
            interoperatibility_array.push(InteroperabilityField::from(
                slice[chunk_start_idx..chunk_start_idx + INTEROPERABILITY_FIELD_SIZE].as_ref(),
                is_little_endian,
            ));
            chunk_start_idx += INTEROPERABILITY_FIELD_SIZE;
        }

        Self {
            number_of_fields: (slice[0], slice[1]),
            interoperability_arrays: interoperatibility_array,
            is_little_endian,
        }
    }

    fn get_array_as_string(&self) -> String {
        let mut res = String::from("[\n");
        let last = self.interoperability_arrays.len() - 1;
        for (i, interop) in self.interoperability_arrays.iter().enumerate() {
            if i == last {
                res.push_str(format!("{}", interop).as_str());
            } else {
                res.push_str(format!("{},\n", interop).as_str());
            }
        }
        res.push_str("\n\t]");

        res
    }

    pub fn get_offset_to_next_ifd(&self) -> usize {
        2 + if self.is_little_endian {
            u8_2_to_u16_le(self.number_of_fields) as usize
        } else {
            u8_2_to_u16_be(self.number_of_fields) as usize
        } * INTEROPERABILITY_FIELD_SIZE
            + 4
    }

    pub fn get_offset_for_tag(&self, tag: usize) -> Option<usize> {
        self.interoperability_arrays
            .iter()
            .find(|interop| interop.get_tag() == tag)
            .map(|interop| interop.get_value_offset())
    }

    pub fn get_interop_for_tag(&self, tag: usize) -> Option<&InteroperabilityField> {
        self.interoperability_arrays
            .iter()
            .find(|interop| interop.get_tag() == tag)
    }

    pub fn get_as_string(&self) -> String {
        format!(
            "IFD {{\n\tNumber of fields: {},\n\tinteroperability: {}\n}}",
            if self.is_little_endian {
                u8_2_to_u16_le(self.number_of_fields)
            } else {
                u8_2_to_u16_be(self.number_of_fields)
            },
            self.get_array_as_string(),
        )
    }
}

pub struct InteroperabilityField {
    tag: (u8, u8),
    data_type: (u8, u8),
    count: (u8, u8, u8, u8),
    value_offset: (u8, u8, u8, u8),
    // Not defined by the spec
    is_little_endian: bool,
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

    pub fn get_tag(&self) -> usize {
        if self.is_little_endian {
            u8_2_to_u16_le(self.tag) as usize
        } else {
            u8_2_to_u16_be(self.tag) as usize
        }
    }

    pub fn get_data_type(&self) -> ExifTypes {
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

    pub fn get_value_offset(&self) -> usize {
        if self.is_little_endian {
            u8_4_to_u32_le(self.value_offset) as usize
        } else {
            u8_4_to_u32_be(self.value_offset) as usize
        }
    }

    pub fn get_value(&self) -> Vec<u8> {
        Vec::new()
    }

    fn get_type_as_string(&self) -> String {
        match self.get_data_type() {
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

    pub fn get_count_as_string(&self) -> String {
        if self.is_little_endian {
            // TODO: Use the value instead of the count
            let count = u8_4_to_u32_le(self.count);
            match self.get_data_type() {
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

impl fmt::Display for TIFFHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_as_string())
    }
}

impl fmt::Display for Ifd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_as_string())
    }
}

impl fmt::Display for InteroperabilityField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\t\t{{
                    tag: {} {} => {},
                    type: {} {} => {} ({}),
                    count: {} {} {} {} => {},
                    value_offset: {} {} {} {} => {}\n\t\t}}",
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
            self.get_type_as_string(),
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
