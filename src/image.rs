pub enum ImageType {
    Jpeg,
    Png,
}

#[allow(dead_code)]
pub trait Image {
    fn from(slice: Vec<u8>) -> Self;

    fn get_infos_as_string(&self) -> String;
    fn print_all_tags(&self);
}

// See https://en.wikipedia.org/wiki/List_of_file_signatures
pub fn get_image_type_for(slice: &[u8]) -> Option<ImageType> {
    if slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xDB].as_ref())  // ÿØÿÛ
        // ÿØÿà␀␐JFIF␀␁,
        || slice.starts_with(
            vec![
                0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
            ]
            .as_ref(), 
        )
        // ÿØÿî
        || slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xEE].as_ref()) 
        // ÿØÿà␀␐JFIF␀␁ 
        || slice.starts_with(
            vec![
                0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x3F, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00,
            ]
            .as_ref(),
        )
        // FF D8 FF E1 ?? ?? 45 78 69 66 00 00
        || (
            slice.len()>=12
            && slice.starts_with(vec![0xFF ,0xD8 ,0xFF ,0xE1].as_ref())
            && slice[6..13].starts_with(vec![0x45 ,0x78 ,0x69 ,0x66 ,0x00 ,0x00].as_ref())
        )
        // ÿØÿà
        || slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xE0].as_ref() )
    {
        return Some(ImageType::Jpeg);
    } else if slice.starts_with(vec![0x49, 0x48, 0x44, 0x52].as_ref()) {
        return Some(ImageType::Png);
    }
    None
}
