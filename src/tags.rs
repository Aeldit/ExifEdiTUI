#[derive(Debug)]
pub enum Tags {
    // Image Data Structure
    ImageWidth,
    ImageLength,
    BitsPerSample,
    Compression,
    PhotometricInterpretation,
    Orientation,
    SamplesPerPixel,
    PlanarConfiguration,
    YCbCrSubSampling,
    YCbCrPositioning,
    XResolution,
    YResolution,
    ResolutionUnit,

    // Offset Recording
    StripOffsets,
    RowsPerStrip,
    StripByteCounts,
    JPEGInterchangeFormat,
    JPEGInterchangeFormatLength,

    // Image Data Characteristics
    TransferFunction,
    WhitePoint,
    PrimaryChromaticities,
    YCbCrCoefficients,
    ReferenceBlackWhite,

    // Other
    DateTime,
    ImageDescription,
    Make,
    Model,
    Software,
    Artist,
    Copyright,
}

pub fn get_usize_for_tag(tag: Tags) -> usize {
    match tag {
        Tags::ImageWidth => 256,
        Tags::ImageLength => 257,
        Tags::BitsPerSample => 258,
        Tags::Compression => 259,
        Tags::PhotometricInterpretation => 262,
        Tags::Orientation => 274,
        Tags::SamplesPerPixel => 277,
        Tags::PlanarConfiguration => 284,
        Tags::YCbCrSubSampling => 530,
        Tags::YCbCrPositioning => 531,
        Tags::XResolution => 282,
        Tags::YResolution => 283,
        Tags::ResolutionUnit => 296,
        Tags::StripOffsets => 273,
        Tags::RowsPerStrip => 278,
        Tags::StripByteCounts => 279,
        Tags::JPEGInterchangeFormat => 513,
        Tags::JPEGInterchangeFormatLength => 514,
        Tags::TransferFunction => 301,
        Tags::WhitePoint => 318,
        Tags::PrimaryChromaticities => 319,
        Tags::YCbCrCoefficients => 529,
        Tags::ReferenceBlackWhite => 532,
        Tags::DateTime => 306,
        Tags::ImageDescription => 270,
        Tags::Make => 271,
        Tags::Model => 272,
        Tags::Software => 305,
        Tags::Artist => 315,
        Tags::Copyright => 33432,
    }
}
