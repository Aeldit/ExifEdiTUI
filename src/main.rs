use std::{env::args, fs};

mod arrays;
pub mod exif;
pub mod tags;
use image::{get_image_type_for, Image, ImageType};

mod image;

mod jpeg;
use jpeg::Jpeg;

//mod png;
//use png::Png;

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

    let img_type = match get_image_type_for(img_contents.as_ref()) {
        Some(img_type) => img_type,
        None => {
            eprintln!("Unsupported image type");
            return Ok(());
        }
    };

    let img = match img_type {
        ImageType::Jpeg => <Jpeg as Image>::from(img_contents),
        ImageType::Png => return Ok(()), //<Png as Image>::from(img_contents),
    };
    img.print_all_tags();

    Ok(())
}
