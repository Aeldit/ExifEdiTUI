use std::{env::args, fs};

mod png;
use png::{ExifChunk, EXIF_CHUNK_SIZE};
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

    let exif_magic = vec![0x65, 0x58, 0x49, 0x66]; // eXIf
    let exif_chunk_start = match index_of_sub_array(img_contents.clone(), exif_magic.clone()) {
        Some(magic_start) => magic_start + exif_magic.len(),
        None => return,
    };
    let exif_chunk = ExifChunk::from(
        img_contents[exif_chunk_start..exif_chunk_start + EXIF_CHUNK_SIZE].as_ref(),
    );
    println!("{}", exif_chunk);

    let mut next_idx = exif_chunk_start + EXIF_CHUNK_SIZE + 4;
    for _ in 0..4 {
        println!(
            "{}",
            ExifChunk::from(img_contents[next_idx..next_idx + EXIF_CHUNK_SIZE].as_ref())
        );
        next_idx += EXIF_CHUNK_SIZE + 4;
    }

    // PNG
    /*let png_magic = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let image_type = if img_contents.starts_with(&png_magic) {
        ImagesTypes::Png
    } else {
        ImagesTypes::Jpeg
    };

    println!("{:?}", image_type);

    let ihdr_magic = vec![0x49, 0x48, 0x44, 0x52];
    let ihdr_chunk_start_idx = match index_of_sub_array(img_contents.clone(), ihdr_magic.clone()) {
        Some(magic_start) => magic_start + ihdr_magic.clone().len(),
        None => return,
    };

    let ihdr_chunk = Ihdr::from(
        img_contents[ihdr_chunk_start_idx..ihdr_chunk_start_idx + IHDR_CHUNK_SIZE].as_ref(),
    );
    println!("{}", ihdr_chunk);*/
}
