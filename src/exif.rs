use core::fmt;

use crate::formats::ImageFormat;
use crate::tags::{
    get_ascii_string_for_tag, get_byte_string_for_tag, get_short_string_for_tag,
    get_undefined_string_for_tag, Tag, Tags,
};

use crate::arrays::{get_tuples_vec_as_string, get_vec_as_string, index_of_sub_array};

// In bytes
pub const TIFF_HEADER_SIZE: usize = 8;
pub const INTEROPERABILITY_FIELD_SIZE: usize = 12;

pub struct ExifImage {
    //image_format: ImageFormat,
    tiff: TIFFHeader,
    ifd_0: IFD,
    ifd_exif: IFD,
    ifd_gps: IFD,
    //ifd_1: IFD,
    slice: Vec<u8>,
}

impl ExifImage {
    pub fn from(img_contents: Vec<u8>, img_format: ImageFormat) -> Self {
        let exif_identifier_code = match img_format {
            ImageFormat::Jpeg => vec![0x45, 0x78, 0x69, 0x66], // Exif
            ImageFormat::Png => vec![0x65, 0x58, 0x49, 0x66],  // eXIf
        };
        let exif_chunk_start =
            match index_of_sub_array(img_contents.clone(), exif_identifier_code.clone()) {
                Some(magic_start) => {
                    magic_start
                        + exif_identifier_code.len()
                        + if img_format == ImageFormat::Jpeg {
                            2
                        } else {
                            0
                        }
                }
                None => panic!("Couldn't get the start of the exif chunk"),
            };
        let tiff = TIFFHeader::from(
            img_contents[exif_chunk_start..exif_chunk_start + TIFF_HEADER_SIZE].as_ref(),
        );
        let is_little_endian = tiff.is_little_endian;

        let ifd_0_start = exif_chunk_start + TIFF_HEADER_SIZE + tiff.zero_th_ifd_offset as usize;
        println!("{}", tiff.zero_th_ifd_offset);
        let ifd_0 = IFD::from(img_contents[ifd_0_start..].as_ref(), is_little_endian);

        let ifd_exif_start = match ifd_0.get_offset_for_tag(Tags::ExifOffset) {
            Some(idf_exif_start) => exif_chunk_start + idf_exif_start,
            None => panic!("Exif IFD start not found"),
        };
        let ifd_exif = IFD::from(img_contents[ifd_exif_start..].as_ref(), is_little_endian);

        let ifd_gps_start = match ifd_0.get_offset_for_tag(Tags::GPSOffset) {
            Some(idf_gps_start) => exif_chunk_start + idf_gps_start,
            None => panic!("GPS IFD start not found"),
        };
        let ifd_gps = IFD::from(img_contents[ifd_gps_start..].as_ref(), is_little_endian);

        Self {
            tiff,
            ifd_0,
            ifd_exif,
            ifd_gps,
            slice: Vec::from(img_contents[exif_chunk_start..].as_ref()),
        }
    }

    pub fn get_infos_as_string(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n",
            self.tiff.get_as_string(),
            self.ifd_0.get_as_string(),
            self.ifd_exif.get_as_string(),
            self.ifd_gps.get_as_string(),
        )
    }

    pub fn print_all_tags(&self) {
        self.ifd_0.print_all_tags(self.slice.as_slice());
        self.ifd_exif.print_all_tags(self.slice.as_slice());
        self.ifd_gps.print_all_tags(self.slice.as_slice());
    }
}

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
    GPS,
}

pub struct TIFFHeader {
    // We will need them later, when the editing is implemented
    // byte_order: [u8; 2],
    // fixed: [u8; 2],
    // ifd_offset: [u8; 4], // If = 8 => followed directly by the 0th IFD
    // Not in the spec
    pub is_little_endian: bool,
    pub zero_th_ifd_offset: u32,
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

        let byte_order: [u8; 2] = slice[0..2].try_into().unwrap();
        let ifd_offset = slice[4..8].try_into().unwrap();
        let is_little_endian = byte_order == [0x49, 0x49];
        let off = if is_little_endian {
            u32::from_le_bytes(ifd_offset)
        } else {
            u32::from_be_bytes(ifd_offset)
        };

        Self {
            is_little_endian,
            zero_th_ifd_offset: if off == 8 { 0 } else { off },
        }
    }

    pub fn get_as_string(&self) -> String {
        format!(
            "TIFF {{ Byte Order: {}, 0th IFD offset: {} }}",
            if self.is_little_endian {
                "Little endian (II)"
            } else {
                "Big endian (MM)"
            },
            self.zero_th_ifd_offset,
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

    pub fn get_interops(&self) -> &Vec<InteroperabilityField> {
        &self.interoperability_arrays
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

    pub fn get_offset_for_tag(&self, tag: Tag) -> Option<usize> {
        self.interoperability_arrays
            .iter()
            .find(|interop| interop.ctag == tag.0)
            .map(|interop| interop.cvalue_offset)
    }

    pub fn get_interop_for_tag(&self, tag: usize) -> Option<&InteroperabilityField> {
        self.interoperability_arrays
            .iter()
            .find(|interop| interop.ctag == tag)
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
            .find(|interop| interop.ctag == tag)
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
    // Calculated values
    ctag: usize,
    cdata_type: ExifTypes,
    ccount: usize,
    cvalue_offset: usize,
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

        let tag: [u8; 2] = slice[0..2].try_into().unwrap();
        let data_type: [u8; 2] = slice[2..4].try_into().unwrap();
        let count: [u8; 4] = slice[4..8].try_into().unwrap();
        let value_offset: [u8; 4] = slice[8..12].try_into().unwrap();

        Self {
            tag,
            data_type,
            count,
            value_offset,
            is_little_endian,
            ctag: if is_little_endian {
                u16::from_le_bytes(tag)
            } else {
                u16::from_be_bytes(tag)
            } as usize,
            cdata_type: match if is_little_endian {
                u16::from_le_bytes(data_type)
            } else {
                u16::from_be_bytes(data_type)
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
            },
            ccount: if is_little_endian {
                u32::from_le_bytes(count)
            } else {
                u32::from_be_bytes(count)
            } as usize,
            cvalue_offset: if is_little_endian {
                u32::from_le_bytes(value_offset)
            } else {
                u32::from_be_bytes(value_offset)
            } as usize,
        }
    }

    pub fn get_value_byte(&self, slice: &[u8]) -> Option<Vec<u8>> {
        if self.cdata_type == ExifTypes::Byte {
            return Some(self.get_bytes(slice));
        }
        None
    }

    pub fn get_value_ascii(&self, slice: &[u8]) -> Option<String> {
        if self.cdata_type == ExifTypes::Ascii {
            return Some(self.get_ascii(slice));
        }
        None
    }

    pub fn get_value_short(&self, slice: &[u8]) -> Option<Vec<u16>> {
        if self.cdata_type == ExifTypes::Short {
            return Some(self.get_shorts(slice));
        }
        None
    }

    pub fn get_value_long(&self, slice: &[u8]) -> Option<Vec<u32>> {
        if self.cdata_type == ExifTypes::Long {
            return Some(self.get_longs(slice));
        }
        None
    }

    pub fn get_value_rational(&self, slice: &[u8]) -> Option<Vec<(u32, u32)>> {
        if self.cdata_type == ExifTypes::Rational {
            return Some(self.get_rationals(slice));
        }
        None
    }

    pub fn get_value_undefined(&self) -> Option<u8> {
        if self.cdata_type == ExifTypes::Undefined {
            return Some(self.get_undefined());
        }
        None
    }

    pub fn get_value_slong(&self, slice: &[u8]) -> Option<Vec<i32>> {
        if self.cdata_type == ExifTypes::Slong {
            return Some(self.get_slongs(slice));
        }
        None
    }

    pub fn get_value_srational(&self, slice: &[u8]) -> Option<Vec<(i32, i32)>> {
        if self.cdata_type == ExifTypes::Byte {
            return Some(self.get_srational(slice));
        }
        None
    }

    fn get_type_as_string(&self) -> String {
        match self.cdata_type {
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

    pub fn get_value_as_string(&self, slice: &[u8]) -> String {
        let tag = Tag(self.ctag);
        match self.cdata_type {
            ExifTypes::Byte => get_byte_string_for_tag(tag, self.ccount, self.get_bytes(slice)),
            ExifTypes::Ascii => get_ascii_string_for_tag(tag, self.ccount, self.get_ascii(slice)),
            ExifTypes::Short => {
                let values = self.get_shorts(slice);
                get_short_string_for_tag(tag, self.ccount, values)
            }
            ExifTypes::Long => {
                format!("{}: {}", tag, get_vec_as_string(self.get_longs(slice)))
            }
            ExifTypes::Rational => {
                format!(
                    "{}: {}",
                    tag,
                    get_tuples_vec_as_string(self.get_rationals(slice))
                )
            }
            ExifTypes::Undefined => get_undefined_string_for_tag(
                tag,
                self.ccount,
                self.value_offset,
                self.cvalue_offset,
                slice,
            ),
            ExifTypes::Slong => format!("{}: {}", tag, get_vec_as_string(self.get_slongs(slice))),
            ExifTypes::Srational => {
                format!(
                    "{}: {}",
                    tag,
                    get_tuples_vec_as_string(self.get_srational(slice))
                )
            }
            ExifTypes::Error => String::from("N/A"),
        }
    }

    fn get_bytes(&self, slice: &[u8]) -> Vec<u8> {
        if self.ccount == 0 {
            return Vec::with_capacity(0);
        }

        if self.ccount <= 4 {
            return Vec::from_iter(self.value_offset[0..self.ccount].iter().copied());
        }

        if self.cvalue_offset + self.ccount <= slice.len() {
            return slice[self.cvalue_offset..self.cvalue_offset + self.ccount].to_vec();
        }
        Vec::new()
    }

    fn get_ascii(&self, slice: &[u8]) -> String {
        let start = self.cvalue_offset;
        let end = start + self.ccount;

        if start >= slice.len() || end >= slice.len() {
            return String::from("ERROR");
        }

        String::from_iter(slice[start..end].iter().map(|b| *b as char))
    }

    fn get_shorts(&self, slice: &[u8]) -> Vec<u16> {
        match self.ccount {
            0 => Vec::with_capacity(0),
            1 => vec![u16::from_le_bytes(
                self.value_offset[0..2].try_into().unwrap(),
            )],
            2 => vec![
                u16::from_le_bytes(self.value_offset[0..2].try_into().unwrap()),
                u16::from_le_bytes(self.value_offset[2..4].try_into().unwrap()),
            ],
            _ => {
                let end_off = self.cvalue_offset + self.ccount * 2;
                if end_off >= slice.len() {
                    return Vec::new();
                }

                Vec::from_iter(
                    slice[self.cvalue_offset..end_off]
                        .rchunks_exact(2)
                        .map(|chunk| u16::from_le_bytes(chunk[0..2].try_into().unwrap())),
                )
            }
        }
    }

    fn get_longs(&self, slice: &[u8]) -> Vec<u32> {
        match self.ccount {
            0 => Vec::with_capacity(0),
            1 => vec![u32::from_le_bytes(self.value_offset)],
            _ => {
                let end_off = self.cvalue_offset + self.ccount * 4;
                if end_off >= slice.len() {
                    return Vec::new();
                }

                Vec::from_iter(
                    slice[self.cvalue_offset..end_off]
                        .rchunks_exact(4)
                        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())),
                )
            }
        }
    }

    fn get_rationals(&self, slice: &[u8]) -> Vec<(u32, u32)> {
        let end_off = self.cvalue_offset + self.ccount * 8;
        if end_off >= slice.len() {
            return Vec::new();
        }

        Vec::from_iter(
            slice[self.cvalue_offset..end_off]
                .rchunks_exact(8)
                .map(|chunk| {
                    (
                        u32::from_le_bytes(chunk[0..4].try_into().unwrap()),
                        u32::from_le_bytes(chunk[4..8].try_into().unwrap()),
                    )
                }),
        )
    }

    fn get_undefined(&self) -> u8 {
        self.value_offset[0]
    }

    fn get_slongs(&self, slice: &[u8]) -> Vec<i32> {
        match self.ccount {
            0 => Vec::with_capacity(0),
            1 => vec![i32::from_le_bytes(self.value_offset)],
            _ => {
                let end_off = self.cvalue_offset + self.ccount * 4;
                if end_off >= slice.len() {
                    return Vec::new();
                }

                Vec::from_iter(
                    slice[self.cvalue_offset..end_off]
                        .rchunks_exact(4)
                        .map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap())),
                )
            }
        }
    }

    fn get_srational(&self, slice: &[u8]) -> Vec<(i32, i32)> {
        let end_off = self.cvalue_offset + self.ccount * 8;
        if end_off >= slice.len() {
            return Vec::new();
        }

        Vec::from_iter(
            slice[self.cvalue_offset..end_off]
                .rchunks_exact(8)
                .map(|chunk| {
                    (
                        i32::from_le_bytes(chunk[0..4].try_into().unwrap()),
                        i32::from_le_bytes(chunk[4..8].try_into().unwrap()),
                    )
                }),
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
            self.ctag,
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
            self.ccount,
            self.value_offset[0],
            self.value_offset[1],
            self.value_offset[2],
            self.value_offset[3],
            self.cvalue_offset,
        )
    }
}
