use core::fmt;

use crate::{u8_array_to_u32, u8_array_to_u64};

pub const IHDR_CHUNK_SIZE: usize = 13;

pub struct PngChunk {
    length: u64,
    chunk_type: Vec<u8>,
    chunk_data: Vec<u8>,
    crc: u64,
}

pub struct Ihdr {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl PngChunk {
    pub fn from(length: &[u8], chunk_type: &[u8], chunk_data: &[u8], crc: &[u8]) -> Self {
        Self {
            length: match u8_array_to_u64(length.to_vec()) {
                Some(len) => len,
                None => panic!("PNG: Error in the length"),
            },
            chunk_type: match chunk_type.len() {
                4 => chunk_type.to_vec(),
                _ => panic!("PNG: Error in the chunk_type"),
            },
            chunk_data: Vec::from(chunk_data),
            crc: match u8_array_to_u64(crc.to_vec()) {
                Some(c) => c,
                None => panic!("PNG: Error in the crc"),
            },
        }
    }
}

impl Ihdr {
    pub fn from(slice: &[u8]) -> Self {
        if slice.len() != IHDR_CHUNK_SIZE {
            panic!(
                "Invalid slice size for the IHDR chunk (expected {} but got {}",
                IHDR_CHUNK_SIZE,
                slice.len()
            );
        }

        Self {
            width: match u8_array_to_u32(slice[0..4].to_vec()) {
                Some(c) => c,
                None => panic!("IHDR: Error in the width"),
            },
            height: match u8_array_to_u32(slice[4..8].to_vec()) {
                Some(c) => c,
                None => panic!("IHDR: Error in the height"),
            },
            bit_depth: slice[8],
            color_type: slice[9],
            compression_method: slice[10],
            filter_method: slice[11],
            interlace_method: slice[12],
        }
    }
}

impl fmt::Display for Ihdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IHDR {{\n\twidth: {},\n\theight: {},\n\tbit_depth: {},\n\tcolor_type: {},\n\tcompression_method: {},\n\tfilter_method: {},\n\tinterlace_method: {}\n}}",
            self.width,
            self.height,
            self.bit_depth,
            self.color_type,
            self.compression_method,
            self.filter_method,
            self.interlace_method
        )
    }
}
