use crate::{
    conversions::index_of_sub_array,
    exif::{TIFFHeader, IFD, TIFF_HEADER_SIZE},
    image::Image,
};

pub struct Jpeg {
    tiff: TIFFHeader,
    ifd_0: IFD,
    ifd_exif: IFD,
}

impl Image for Jpeg {
    fn from(img_contents: Vec<u8>) -> Self {
        let exif_identifier_code_jpeg = vec![0x45, 0x78, 0x69, 0x66]; // Exif
        let exif_chunk_start =
            match index_of_sub_array(img_contents.clone(), exif_identifier_code_jpeg.clone()) {
                Some(magic_start) => magic_start + exif_identifier_code_jpeg.len() + 2,
                None => panic!("Couldn't get the start of the exif chunk"),
            };
        let tiff = TIFFHeader::from(
            img_contents[exif_chunk_start..exif_chunk_start + TIFF_HEADER_SIZE].as_ref(),
        );
        let is_little_endian = tiff.is_little_endian();

        let ifd_0_start = exif_chunk_start + TIFF_HEADER_SIZE + tiff.get_0th_idf_offset() as usize;
        let ifd_0 = IFD::from(img_contents[ifd_0_start..].as_ref(), is_little_endian);

        let idf_exif_start = match ifd_0.get_offset_for_tag(34665) {
            Some(idf_exif_start) => exif_chunk_start + idf_exif_start,
            None => panic!(""),
        };
        let ifd_exif = IFD::from(img_contents[idf_exif_start..].as_ref(), is_little_endian);

        Self {
            tiff,
            ifd_0,
            ifd_exif,
        }
    }

    fn get_infos_as_string(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.tiff.get_as_string(),
            self.ifd_0.get_as_string(),
            self.ifd_exif.get_as_string(),
        )
    }
}
