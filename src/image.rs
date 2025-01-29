pub enum ImageType {
    Jpeg,
}

pub trait Image {
    fn from(slice: Vec<u8>) -> Self;

    fn get_infos_as_string(&self) -> String;
}
