use core::fmt;

use crate::tags::{get_tag_for_usize, Tag, Tagss};

// In bytes
pub const TIFF_HEADER_SIZE: usize = 8;
pub const INTEROPERABILITY_FIELD_SIZE: usize = 12;

/*pub enum MagicBytes {
    Png(),
    Jpeg(),
}*/

#[derive(PartialEq)]
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

pub enum IFDTypes {
    TIFF,
    Exif,
}

pub struct TIFFHeader {
    byte_order: [u8; 2],
    // Not used but in the spec
    // fixed: [u8; 2],
    ifd_offset: [u8; 4], // If = 8 => followed directly by the 0th IFD
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
            byte_order: slice[0..2].try_into().unwrap(),
            ifd_offset: slice[4..8].try_into().unwrap(),
        }
    }

    pub fn is_little_endian(&self) -> bool {
        self.byte_order == [0x49, 0x49]
    }

    pub fn get_0th_idf_offset(&self) -> u32 {
        let off = if self.is_little_endian() {
            u32::from_le_bytes(self.ifd_offset)
        } else {
            u32::from_be_bytes(self.ifd_offset)
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
                let off = self.get_0th_idf_offset();
                if off == 8 {
                    0
                } else {
                    off
                }
            }
        )
    }
}

pub struct IFD {
    pub number_of_fields: [u8; 2],
    pub interoperability_arrays: Vec<InteroperabilityField>, // Vec of size number_of_fields
    // Not in the spec
    is_little_endian: bool,
} // 4 byte offset to the next IFD

impl IFD {
    pub fn from(slice: &[u8], is_little_endian: bool) -> Self {
        if slice.len() < 2 {
            panic!(
                "Expected the slice length to be at least 2 for the number_of_fields, but got {}",
                slice.len()
            );
        }

        let number_of_fields = if is_little_endian {
            u16::from_le_bytes(slice[0..2].try_into().unwrap())
        } else {
            u16::from_be_bytes(slice[0..2].try_into().unwrap())
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
            number_of_fields: slice[0..2].try_into().unwrap(),
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
            u16::from_le_bytes(self.number_of_fields) as usize
        } else {
            u16::from_be_bytes(self.number_of_fields) as usize
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
                u16::from_le_bytes(self.number_of_fields)
            } else {
                u16::from_be_bytes(self.number_of_fields)
            },
            self.get_array_as_string(),
        )
    }

    pub fn get_value_as_string_for_tag(&self, tag: usize, slice: &[u8]) -> String {
        match self
            .interoperability_arrays
            .iter()
            .find(|interop| interop.get_tag() == tag)
        {
            Some(interop) => interop.get_value_as_string(slice),
            None => String::from("Error"),
        }
    }

    pub fn print_all_tags(&self, slice: &[u8]) {
        for interop in &self.interoperability_arrays {
            println!("{}", interop.get_value_as_string(slice))
        }
    }
}

pub struct InteroperabilityField {
    tag: [u8; 2],
    data_type: [u8; 2],
    count: [u8; 4],
    value_offset: [u8; 4],
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
            tag: slice[0..2].try_into().unwrap(),
            data_type: slice[2..4].try_into().unwrap(),
            count: slice[4..8].try_into().unwrap(),
            value_offset: slice[8..12].try_into().unwrap(),
            is_little_endian,
        }
    }

    pub fn get_tag(&self) -> usize {
        if self.is_little_endian {
            u16::from_le_bytes(self.tag) as usize
        } else {
            u16::from_be_bytes(self.tag) as usize
        }
    }

    pub fn get_data_type(&self) -> ExifTypes {
        match if self.is_little_endian {
            u16::from_le_bytes(self.data_type)
        } else {
            u16::from_be_bytes(self.data_type)
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
            u32::from_le_bytes(self.value_offset) as usize
        } else {
            u32::from_be_bytes(self.value_offset) as usize
        }
    }

    pub fn get_value_byte(&self) -> Option<u8> {
        if self.get_data_type() == ExifTypes::Byte {
            return Some(self.get_byte());
        }
        None
    }

    pub fn get_value_ascii(&self, slice: &[u8]) -> Option<String> {
        if self.get_data_type() == ExifTypes::Ascii {
            return Some(self.get_ascii(slice));
        }
        None
    }

    pub fn get_value_short(&self) -> Option<u16> {
        if self.get_data_type() == ExifTypes::Short {
            return Some(self.get_short());
        }
        None
    }

    pub fn get_value_long(&self) -> Option<u32> {
        if self.get_data_type() == ExifTypes::Long {
            return Some(self.get_long());
        }
        None
    }

    pub fn get_value_rational(&self, slice: &[u8]) -> Option<(u32, u32)> {
        if self.get_data_type() == ExifTypes::Rational {
            return Some(self.get_rational(slice));
        }
        None
    }

    pub fn get_value_undefined(&self) -> Option<u8> {
        if self.get_data_type() == ExifTypes::Undefined {
            return Some(self.get_undefined());
        }
        None
    }

    pub fn get_value_slong(&self) -> Option<i32> {
        if self.get_data_type() == ExifTypes::Slong {
            return Some(self.get_slong());
        }
        None
    }

    pub fn get_value_srational(&self, slice: &[u8]) -> Option<(i32, i32)> {
        if self.get_data_type() == ExifTypes::Byte {
            return Some(self.get_srational(slice));
        }
        None
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

    pub fn get_count(&self) -> usize {
        if self.is_little_endian {
            u32::from_le_bytes(self.count) as usize
        } else {
            u32::from_be_bytes(self.count) as usize
        }
    }

    pub fn get_value_as_string(&self, slice: &[u8]) -> String {
        // TODO: Handle cases where the value is an enum
        let tag = Tag(self.get_tag());
        match self.get_data_type() {
            ExifTypes::Byte => format!("{}: {}", tag, self.get_byte()),
            ExifTypes::Ascii => {
                format!("{}: {} {}", tag, self.get_ascii(slice), Tagss::ExifOffset)
            }
            ExifTypes::Short => format!("{}: {}", tag, self.get_short()),
            ExifTypes::Long => format!("{}: {}", tag, self.get_long()),
            ExifTypes::Rational => {
                let rational = self.get_rational(slice);
                format!("{}: {}/{}", tag, rational.0, rational.1)
            }
            ExifTypes::Undefined => format!("{}: {}", tag, self.get_long()),
            ExifTypes::Slong => format!("{}: {}", tag, self.get_slong()),
            ExifTypes::Srational => {
                let srational = self.get_srational(slice);
                format!("{}: {}/{}", tag, srational.0, srational.1)
            }
            ExifTypes::Error => String::from("N/A"),
        }
    }

    fn get_byte(&self) -> u8 {
        self.value_offset[0]
    }

    fn get_ascii(&self, slice: &[u8]) -> String {
        let start = self.get_value_offset();
        let end = start + self.get_count();

        if start >= slice.len() || end >= slice.len() {
            return String::from("ERROR");
        }

        String::from_iter(slice[start..end].iter().map(|b| *b as char))
    }

    fn get_short(&self) -> u16 {
        u32::from_le_bytes(self.value_offset) as u16
    }

    fn get_long(&self) -> u32 {
        u32::from_le_bytes(self.value_offset)
    }

    fn get_rational(&self, slice: &[u8]) -> (u32, u32) {
        if slice.len() < self.get_value_offset() + 8 {
            return (0, 0);
        }

        let val_off = self.get_value_offset();

        (
            u32::from_le_bytes(slice[val_off..val_off + 4].try_into().unwrap()),
            u32::from_le_bytes(slice[val_off + 4..val_off + 8].try_into().unwrap()),
        )
    }

    fn get_undefined(&self) -> u8 {
        self.value_offset[0]
    }

    fn get_slong(&self) -> i32 {
        i32::from_le_bytes(self.value_offset)
    }

    fn get_srational(&self, slice: &[u8]) -> (i32, i32) {
        if slice.len() < self.get_value_offset() + 8 {
            return (0, 0);
        }

        let val_off = self.get_value_offset();

        (
            i32::from_le_bytes(slice[val_off..val_off + 4].try_into().unwrap()),
            i32::from_le_bytes(slice[val_off + 4..val_off + 8].try_into().unwrap()),
        )
    }
}

impl fmt::Display for TIFFHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_as_string())
    }
}

impl fmt::Display for IFD {
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
            self.tag[0],
            self.tag[1],
            self.get_tag(),
            self.data_type[0],
            self.data_type[1],
            if self.is_little_endian {
                u16::from_le_bytes(self.data_type)
            } else {
                u16::from_be_bytes(self.data_type)
            },
            self.get_type_as_string(),
            self.count[0],
            self.count[1],
            self.count[2],
            self.count[3],
            self.get_count(),
            self.value_offset[0],
            self.value_offset[1],
            self.value_offset[2],
            self.value_offset[3],
            self.get_value_offset(),
        )
    }
}
