use std::{env::args, fs};

mod conversions;
pub mod exif;
use conversions::*;
use image::{Image, ImageType};

mod image;

mod jpeg;
use jpeg::Jpeg;

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

    let img_type = if img_contents.starts_with(
        vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        ]
        .as_ref(),
    ) || img_contents.starts_with(vec![0xFF, 0xD8, 0xFF, 0xEE].as_ref())
    {
        ImageType::Jpeg
    } else {
        return;
    };

    let img = match img_type {
        ImageType::Jpeg => <Jpeg as Image>::from(img_contents),
    };

    println!("{}", img.get_infos_as_string());

    /*let png_exif_magic = vec![0x65, 0x58, 0x49, 0x66]; // eXIf
        if let Some(interop) = ifd_exif.get_interop_for_tag(40961) {
        //println!("{}", interop.get_value())
    };*/
}
