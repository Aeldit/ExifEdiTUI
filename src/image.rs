pub enum ImageType {
    Jpeg,
}

#[allow(dead_code)]
pub trait Image {
    fn from(slice: Vec<u8>) -> Self;

    fn get_infos_as_string(&self) -> String;
    fn print_all_tags(&self);
}

pub fn get_image_type_for(slice: &[u8]) -> Option<ImageType> {
    if slice.starts_with(
        vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        ]
        .as_ref(),
    ) || slice.starts_with(vec![0xFF, 0xD8, 0xFF, 0xEE].as_ref())
        || slice.starts_with(
            vec![
                0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x3F, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00,
            ]
            .as_ref(),
        )
    {
        return Some(ImageType::Jpeg);
    }
    None
}
