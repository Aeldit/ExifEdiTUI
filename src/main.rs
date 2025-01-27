use std::{env::args, fs};

mod exif;
use exif::{ExifChunk, EXIF_CHUNK_SIZE};
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
    for _ in 0..8 {
        println!(
            "{}",
            ExifChunk::from(img_contents[next_idx..next_idx + EXIF_CHUNK_SIZE].as_ref())
        );
        next_idx += EXIF_CHUNK_SIZE + 4;
    }
}
