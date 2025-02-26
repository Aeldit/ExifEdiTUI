#[derive(PartialEq)]
pub enum ImageFormat {
    Jpeg,
    Png,
}

// See https://en.wikipedia.org/wiki/List_of_file_signatures
pub fn get_image_format_for(slice: &[u8]) -> Option<ImageFormat> {
    if is_jpeg(slice) {
        Some(ImageFormat::Jpeg)
    } else if is_png(slice) {
        Some(ImageFormat::Png)
    } else {
        None
    }
}

fn is_jpeg(slice: &[u8]) -> bool {
    slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xDB].as_ref())
        || slice.starts_with(
            vec![
                0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
            ]
            .as_ref(),
        )
        || slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xEE].as_ref())
        || slice.starts_with(
            vec![
                0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x3F, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00,
            ]
            .as_ref(),
        )
        || (slice.len() >= 12
            && slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xE1].as_ref())
            && slice[6..13].starts_with(vec![0x45, 0x78, 0x69, 0x66, 0x00, 0x00].as_ref()))
        || slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xE0].as_ref())
}

fn is_png(slice: &[u8]) -> bool {
    slice.starts_with(vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A].as_ref())
}
