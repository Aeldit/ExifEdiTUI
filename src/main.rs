use std::{env::args, fs};

mod exif;
use exif::{Ifd, TIFFHeader, TIFF_HEADER_SIZE};
mod conversions;
use conversions::*;

/*#[derive(Debug)]
enum ImagesTypes {
    Png,
    Jpeg,
}*/

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

    //let png_exif_magic = vec![0x65, 0x58, 0x49, 0x66]; // eXIf
    let jpg_exif_magic = vec![0x45, 0x78, 0x69, 0x66]; // Exif
    let exif_chunk_start = match index_of_sub_array(img_contents.clone(), jpg_exif_magic.clone()) {
        Some(magic_start) => magic_start + jpg_exif_magic.len() + 2,
        None => return,
    };
    let tiff = TIFFHeader::from(
        img_contents[exif_chunk_start..exif_chunk_start + TIFF_HEADER_SIZE].as_ref(),
    );
    println!("{}", tiff);
    let is_little_endian = tiff.is_little_endian();

    let ifd_0_start = exif_chunk_start + TIFF_HEADER_SIZE + tiff.get_0th_idf_offset() as usize;
    let ifd_0 = Ifd::from(img_contents[ifd_0_start..].as_ref(), is_little_endian);
    println!("{}", ifd_0);
}
