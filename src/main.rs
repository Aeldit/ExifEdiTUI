use std::{env::args, fs};

mod arrays;
pub mod exif;
mod formats;
pub mod tags;

use exif::ExifImage;
use formats::get_image_format_for;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Ok(());
    }

    let img_path = match args.get(1) {
        Some(img_path) => img_path,
        None => return Ok(()),
    };

    let img_contents = fs::read(img_path)?;

    let img_format = match get_image_format_for(img_contents.as_ref()) {
        Some(img_type) => img_type,
        None => {
            eprintln!("Unsupported image type");
            return Ok(());
        }
    };

    let img = ExifImage::from(img_contents, img_format);
    img.print_all_tags();

    Ok(())
}
