use core::fmt;

#[derive(Debug)]
pub enum Tags {
    // Special IFDs
    ExifOffset,
    GPSOffset,
    Interoperability,

    // TIFF
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

    ////////////////////////////////////////////////////////////////////////////
    // Exif
    ////////////////////////////////////////////////////////////////////////////
    // Tags Relating to Version
    ExifVersion,
    FlashpixVersion,

    // Tags Relating to ColorSpace
    ColorSpace,
    Gamma,

    // Tags Relating to Image Configuration
    ComponentsConfiguration,
    CompressedBitsPerPixel,
    PixelXDimension,
    PixelYDimension,

    // Tags Relating to User Information
    MakerNote,
    UserComment,

    // Tag Relating to Related File Information
    RelatedSoundFile,

    // Tags Relating to Date and Time
    DateTimeOriginal,
    DateTimeDigitized,
    OffsetTime,
    OffsetTimeOriginal,
    OffsetTimeDigitized,
    SubSecTime,
    SubSecTimeOriginal,
    SubSecTimeDigitized,

    // Tags Relating to Picture-Taking Conditions
    ExposureTime,
    FNumber,
    ExposureProgram,
    SpectralSensitivity,
    PhotographicSensitivity,
    OECF,
    SensitivityType,
    StandardOutputSensitivity,
    RecommendedExposureIndex,
    ISOSpeed,
    ISOSpeedLatitudeyyy,
    ISOSpeedLatitudezzz,
    ShutterSpeedValue,
    ApertureValue,
    BrightnessValue,
    ExposureBiasValue,
    MaxApertureValue,
    SubjectDistance,
    MeteringMode,
    LightSource,
    Flash,
    FocalLength,
    SubjectArea,
    FlashEnergy,
    SpatialFrequencyResponse,
    FocalPlaneXResolution,
    FocalPlaneYResolution,
    FocalPlaneResolutionUnit,
    SubjectLocation,
    ExposureIndex,
    SensingMethod,
    FileSource,
    SceneType,
    CFAPattern,
    CustomRendered,
    ExposureMode,
    WhiteBalance,
    DigitalZoomRatio,
    FocalLengthIn35mmFilm,
    SceneCaptureType,
    GainControl,
    Contrast,
    Saturation,
    Sharpness,
    DeviceSettingDescription,
    SubjectDistanceRange,
    CompositeImage,
    SourceImageNumberOfCompositeImage,
    SourceExposureTimesOfCompositeImage,

    // Tags Relating to shooting situation
    Temperature,
    Humidity,
    Pressure,
    WaterDepth,
    Acceleration,
    CameraElevationAngle,

    // Other Tags
    ImageUniqueID,
    CameraOwnerName,
    BodySerialNumber,
    LensSpecification,
    LensMake,
    LensModel,
    LensSerialNumber,
    ////////////////////////////////////////////////////////////////////////////
    // GPS
    ////////////////////////////////////////////////////////////////////////////
    /*GPSVersionID,
    GPSLatitudeRef,
    GPSLatitude,
    GPSLongitudeRef,
    GPSLongitude,
    GPSAltitudeRef,
    GPSAltitude,
    GPSTimeStamp,
    GPSSatellites,
    GPSStatus,
    GPSMeasureMode,
    GPSDOP,
    GPSSpeedRef,
    GPSSpeed,
    GPSTrackRef,
    GPSTrack,
    GPSImgDirectionRef,
    GPSImgDirection,
    GPSMapDatum,
    GPSDestLatitudeRef,
    GPSDestLatitude,
    GPSDestLongitudeRef,
    GPSDestLongitude,
    GPSDestBearingRef,
    GPSDestBearing,
    GPSDestDistanceRef,
    GPSDestDistance,
    GPSProcessingMethod,
    GPSAreaInformation,
    GPSDateStamp,
    GPSDifferential,
    GPSHPositioningError,*/
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Tagss {
    use super::Tag;

    // Special IFDs
    pub const ExifOffset: Tag = Tag(34665);
    pub const GPSOffset: Tag = Tag(34853);
    pub const Interoperability: Tag = Tag(40965);

    pub const ImageWidth: Tag = Tag(256);
    pub const ImageLength: Tag = Tag(257);
    pub const BitsPerSample: Tag = Tag(258);
    pub const Compression: Tag = Tag(259);
    pub const PhotometricInterpretation: Tag = Tag(262);
    pub const Orientation: Tag = Tag(274);
    pub const SamplesPerPixel: Tag = Tag(277);
    pub const PlanarConfiguration: Tag = Tag(284);
    pub const YCbCrSubSampling: Tag = Tag(530);
    pub const YCbCrPositioning: Tag = Tag(531);
    pub const XResolution: Tag = Tag(282);
    pub const YResolution: Tag = Tag(283);
    pub const ResolutionUnit: Tag = Tag(296);
    pub const StripOffsets: Tag = Tag(273);
    pub const RowsPerStrip: Tag = Tag(278);
    pub const StripByteCounts: Tag = Tag(279);
    pub const JPEGInterchangeFormat: Tag = Tag(513);
    pub const JPEGInterchangeFormatLength: Tag = Tag(514);
    pub const TransferFunction: Tag = Tag(301);
    pub const WhitePoint: Tag = Tag(318);
    pub const PrimaryChromaticities: Tag = Tag(319);
    pub const YCbCrCoefficients: Tag = Tag(529);
    pub const ReferenceBlackWhite: Tag = Tag(532);
    pub const DateTime: Tag = Tag(306);
    pub const ImageDescription: Tag = Tag(270);
    pub const Make: Tag = Tag(271);
    pub const Model: Tag = Tag(272);
    pub const Software: Tag = Tag(305);
    pub const Artist: Tag = Tag(315);
    pub const Copyright: Tag = Tag(33432);
    pub const ExifVersion: Tag = Tag(36864);
    pub const FlashpixVersion: Tag = Tag(40960);
    pub const ColorSpace: Tag = Tag(40961);
    pub const Gamma: Tag = Tag(42240);
    pub const ComponentsConfiguration: Tag = Tag(37121);
    pub const CompressedBitsPerPixel: Tag = Tag(37122);
    pub const PixelXDimension: Tag = Tag(40962);
    pub const PixelYDimension: Tag = Tag(40963);
    pub const MakerNote: Tag = Tag(37500);
    pub const UserComment: Tag = Tag(37510);
    pub const RelatedSoundFile: Tag = Tag(40964);
    pub const DateTimeOriginal: Tag = Tag(36867);
    pub const DateTimeDigitized: Tag = Tag(36868);
    pub const OffsetTime: Tag = Tag(36880);
    pub const OffsetTimeOriginal: Tag = Tag(36881);
    pub const OffsetTimeDigitized: Tag = Tag(36882);
    pub const SubSecTime: Tag = Tag(37520);
    pub const SubSecTimeOriginal: Tag = Tag(37521);
    pub const SubSecTimeDigitized: Tag = Tag(37522);
    pub const ExposureTime: Tag = Tag(33434);
    pub const FNumber: Tag = Tag(33437);
    pub const ExposureProgram: Tag = Tag(34850);
    pub const SpectralSensitivity: Tag = Tag(34852);
    pub const PhotographicSensitivity: Tag = Tag(34855);
    pub const OECF: Tag = Tag(34856);
    pub const SensitivityType: Tag = Tag(34864);
    pub const StandardOutputSensitivity: Tag = Tag(34865);
    pub const RecommendedExposureIndex: Tag = Tag(34866);
    pub const ISOSpeed: Tag = Tag(34867);
    pub const ISOSpeedLatitudeyyy: Tag = Tag(34868);
    pub const ISOSpeedLatitudezzz: Tag = Tag(34869);
    pub const ShutterSpeedValue: Tag = Tag(37377);
    pub const ApertureValue: Tag = Tag(37378);
    pub const BrightnessValue: Tag = Tag(37379);
    pub const ExposureBiasValue: Tag = Tag(37380);
    pub const MaxApertureValue: Tag = Tag(37381);
    pub const SubjectDistance: Tag = Tag(37382);
    pub const MeteringMode: Tag = Tag(37383);
    pub const LightSource: Tag = Tag(37384);
    pub const Flash: Tag = Tag(37385);
    pub const FocalLength: Tag = Tag(37386);
    pub const SubjectArea: Tag = Tag(37396);
    pub const FlashEnergy: Tag = Tag(41483);
    pub const SpatialFrequencyResponse: Tag = Tag(41484);
    pub const FocalPlaneXResolution: Tag = Tag(41486);
    pub const FocalPlaneYResolution: Tag = Tag(41487);
    pub const FocalPlaneResolutionUnit: Tag = Tag(41488);
    pub const SubjectLocation: Tag = Tag(41492);
    pub const ExposureIndex: Tag = Tag(41493);
    pub const SensingMethod: Tag = Tag(41495);
    pub const FileSource: Tag = Tag(41728);
    pub const SceneType: Tag = Tag(41729);
    pub const CFAPattern: Tag = Tag(41730);
    pub const CustomRendered: Tag = Tag(41985);
    pub const ExposureMode: Tag = Tag(41986);
    pub const WhiteBalance: Tag = Tag(41987);
    pub const DigitalZoomRatio: Tag = Tag(41988);
    pub const FocalLengthIn35mmFilm: Tag = Tag(41989);
    pub const SceneCaptureType: Tag = Tag(41990);
    pub const GainControl: Tag = Tag(41991);
    pub const Contrast: Tag = Tag(41992);
    pub const Saturation: Tag = Tag(41993);
    pub const Sharpness: Tag = Tag(41994);
    pub const DeviceSettingDescription: Tag = Tag(41995);
    pub const SubjectDistanceRange: Tag = Tag(41996);
    pub const CompositeImage: Tag = Tag(42080);
    pub const SourceImageNumberOfCompositeImage: Tag = Tag(42081);
    pub const SourceExposureTimesOfCompositeImage: Tag = Tag(42082);
    pub const Temperature: Tag = Tag(37888);
    pub const Humidity: Tag = Tag(37889);
    pub const Pressure: Tag = Tag(37890);
    pub const WaterDepth: Tag = Tag(37891);
    pub const Acceleration: Tag = Tag(37892);
    pub const CameraElevationAngle: Tag = Tag(37893);
    pub const ImageUniqueID: Tag = Tag(42016);
    pub const CameraOwnerName: Tag = Tag(42032);
    pub const BodySerialNumber: Tag = Tag(42033);
    pub const LensSpecification: Tag = Tag(42034);
    pub const LensMake: Tag = Tag(42035);
    pub const LensModel: Tag = Tag(42036);
    pub const LensSerialNumber: Tag = Tag(42037);
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tagss::ExifOffset => "ExifOffset",
                Tagss::ImageWidth => "ImageWidth",
                Tagss::ImageLength => "ImageLength",
                Tagss::BitsPerSample => "BitsPerSample",
                Tagss::Compression => "Compression",
                Tagss::PhotometricInterpretation => "PhotometricInterpretation",
                Tagss::Orientation => "Orientation",
                Tagss::SamplesPerPixel => "SamplesPerPixel",
                Tagss::PlanarConfiguration => "PlanarConfiguration",
                Tagss::YCbCrSubSampling => "YCbCrSubSampling",
                Tagss::YCbCrPositioning => "YCbCrPositioning",
                Tagss::XResolution => "",
                Tagss::YResolution => "",
                Tagss::ResolutionUnit => "",
                Tagss::StripOffsets => "",
                Tagss::RowsPerStrip => "",
                Tagss::StripByteCounts => "",
                Tagss::JPEGInterchangeFormat => "",
                Tagss::JPEGInterchangeFormatLength => "",
                Tagss::TransferFunction => "",
                Tagss::WhitePoint => "",
                Tagss::PrimaryChromaticities => "",
                Tagss::YCbCrCoefficients => "",
                Tagss::ReferenceBlackWhite => "",
                Tagss::DateTime => "",
                Tagss::ImageDescription => "",
                Tagss::Make => "",
                Tagss::Model => "",
                Tagss::Software => "",
                Tagss::Artist => "",
                Tagss::Copyright => "",
                Tagss::ExifVersion => "",
                Tagss::FlashpixVersion => "",
                Tagss::ColorSpace => "",
                Tagss::Gamma => "",
                Tagss::ComponentsConfiguration => "",
                Tagss::CompressedBitsPerPixel => "",
                Tagss::PixelXDimension => "",
                Tagss::PixelYDimension => "",
                Tagss::MakerNote => "",
                Tagss::UserComment => "",
                Tagss::RelatedSoundFile => "",
                Tagss::DateTimeOriginal => "",
                Tagss::DateTimeDigitized => "",
                Tagss::OffsetTime => "",
                Tagss::OffsetTimeOriginal => "",
                Tagss::OffsetTimeDigitized => "",
                Tagss::SubSecTime => "",
                Tagss::SubSecTimeOriginal => "",
                Tagss::SubSecTimeDigitized => "",
                Tagss::ExposureTime => "",
                Tagss::FNumber => "",
                Tagss::ExposureProgram => "",
                Tagss::SpectralSensitivity => "",
                Tagss::PhotographicSensitivity => "",
                Tagss::OECF => "",
                Tagss::SensitivityType => "",
                Tagss::StandardOutputSensitivity => "",
                Tagss::RecommendedExposureIndex => "",
                Tagss::ISOSpeed => "",
                Tagss::ISOSpeedLatitudeyyy => "",
                Tagss::ISOSpeedLatitudezzz => "",
                Tagss::ShutterSpeedValue => "",
                Tagss::ApertureValue => "",
                Tagss::BrightnessValue => "",
                Tagss::ExposureBiasValue => "",
                Tagss::MaxApertureValue => "",
                Tagss::SubjectDistance => "",
                Tagss::MeteringMode => "",
                Tagss::LightSource => "",
                Tagss::Flash => "",
                Tagss::FocalLength => "",
                Tagss::SubjectArea => "",
                Tagss::FlashEnergy => "",
                Tagss::SpatialFrequencyResponse => "",
                Tagss::FocalPlaneXResolution => "",
                Tagss::FocalPlaneYResolution => "",
                Tagss::FocalPlaneResolutionUnit => "",
                Tagss::SubjectLocation => "",
                Tagss::ExposureIndex => "",
                Tagss::SensingMethod => "",
                Tagss::FileSource => "",
                Tagss::SceneType => "",
                Tagss::CFAPattern => "",
                Tagss::CustomRendered => "",
                Tagss::ExposureMode => "",
                Tagss::WhiteBalance => "",
                Tagss::DigitalZoomRatio => "",
                Tagss::FocalLengthIn35mmFilm => "",
                Tagss::SceneCaptureType => "",
                Tagss::GainControl => "",
                Tagss::Contrast => "",
                Tagss::Saturation => "",
                Tagss::Sharpness => "",
                Tagss::DeviceSettingDescription => "",
                Tagss::SubjectDistanceRange => "",
                Tagss::CompositeImage => "",
                Tagss::SourceImageNumberOfCompositeImage => "",
                Tagss::SourceExposureTimesOfCompositeImage => "",
                Tagss::Temperature => "",
                Tagss::Humidity => "",
                Tagss::Pressure => "",
                Tagss::WaterDepth => "",
                Tagss::Acceleration => "",
                Tagss::CameraElevationAngle => "",
                Tagss::ImageUniqueID => "",
                Tagss::CameraOwnerName => "",
                Tagss::BodySerialNumber => "",
                Tagss::LensSpecification => "",
                Tagss::LensMake => "",
                Tagss::LensModel => "",
                Tagss::LensSerialNumber => "",
                _ => "unknown",
            }
        )
    }
}

#[derive(PartialEq)]
pub struct Tag(pub usize);

pub fn get_usize_for_tag(tag: Tags) -> usize {
    match tag {
        Tags::ExifOffset => 34665,
        Tags::GPSOffset => 34853,
        Tags::Interoperability => 40965,
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
        // Exif
        Tags::ExifVersion => 36864,
        Tags::FlashpixVersion => 40960,
        Tags::ColorSpace => 40961,
        Tags::Gamma => 42240,
        Tags::ComponentsConfiguration => 37121,
        Tags::CompressedBitsPerPixel => 37122,
        Tags::PixelXDimension => 40962,
        Tags::PixelYDimension => 40963,
        Tags::MakerNote => 37500,
        Tags::UserComment => 37510,
        Tags::RelatedSoundFile => 40964,
        Tags::DateTimeOriginal => 36867,
        Tags::DateTimeDigitized => 36868,
        Tags::OffsetTime => 36880,
        Tags::OffsetTimeOriginal => 36881,
        Tags::OffsetTimeDigitized => 36882,
        Tags::SubSecTime => 37520,
        Tags::SubSecTimeOriginal => 37521,
        Tags::SubSecTimeDigitized => 37522,
        Tags::ExposureTime => 33434,
        Tags::FNumber => 33437,
        Tags::ExposureProgram => 34850,
        Tags::SpectralSensitivity => 34852,
        Tags::PhotographicSensitivity => 34855,
        Tags::OECF => 34856,
        Tags::SensitivityType => 34864,
        Tags::StandardOutputSensitivity => 34865,
        Tags::RecommendedExposureIndex => 34866,
        Tags::ISOSpeed => 34867,
        Tags::ISOSpeedLatitudeyyy => 34868,
        Tags::ISOSpeedLatitudezzz => 34869,
        Tags::ShutterSpeedValue => 37377,
        Tags::ApertureValue => 37378,
        Tags::BrightnessValue => 37379,
        Tags::ExposureBiasValue => 37380,
        Tags::MaxApertureValue => 37381,
        Tags::SubjectDistance => 37382,
        Tags::MeteringMode => 37383,
        Tags::LightSource => 37384,
        Tags::Flash => 37385,
        Tags::FocalLength => 37386,
        Tags::SubjectArea => 37396,
        Tags::FlashEnergy => 41483,
        Tags::SpatialFrequencyResponse => 41484,
        Tags::FocalPlaneXResolution => 41486,
        Tags::FocalPlaneYResolution => 41487,
        Tags::FocalPlaneResolutionUnit => 41488,
        Tags::SubjectLocation => 41492,
        Tags::ExposureIndex => 41493,
        Tags::SensingMethod => 41495,
        Tags::FileSource => 41728,
        Tags::SceneType => 41729,
        Tags::CFAPattern => 41730,
        Tags::CustomRendered => 41985,
        Tags::ExposureMode => 41986,
        Tags::WhiteBalance => 41987,
        Tags::DigitalZoomRatio => 41988,
        Tags::FocalLengthIn35mmFilm => 41989,
        Tags::SceneCaptureType => 41990,
        Tags::GainControl => 41991,
        Tags::Contrast => 41992,
        Tags::Saturation => 41993,
        Tags::Sharpness => 41994,
        Tags::DeviceSettingDescription => 41995,
        Tags::SubjectDistanceRange => 41996,
        Tags::CompositeImage => 42080,
        Tags::SourceImageNumberOfCompositeImage => 42081,
        Tags::SourceExposureTimesOfCompositeImage => 42082,
        Tags::Temperature => 37888,
        Tags::Humidity => 37889,
        Tags::Pressure => 37890,
        Tags::WaterDepth => 37891,
        Tags::Acceleration => 37892,
        Tags::CameraElevationAngle => 37893,
        Tags::ImageUniqueID => 42016,
        Tags::CameraOwnerName => 42032,
        Tags::BodySerialNumber => 42033,
        Tags::LensSpecification => 42034,
        Tags::LensMake => 42035,
        Tags::LensModel => 42036,
        Tags::LensSerialNumber => 42037,
    }
}

pub fn get_tag_for_usize(tag: usize) -> Option<Tags> {
    match tag {
        34665 => Some(Tags::ExifOffset),
        34853 => Some(Tags::GPSOffset),
        40965 => Some(Tags::Interoperability),
        256 => Some(Tags::ImageWidth),
        257 => Some(Tags::ImageLength),
        258 => Some(Tags::BitsPerSample),
        259 => Some(Tags::Compression),
        262 => Some(Tags::PhotometricInterpretation),
        274 => Some(Tags::Orientation),
        277 => Some(Tags::SamplesPerPixel),
        284 => Some(Tags::PlanarConfiguration),
        530 => Some(Tags::YCbCrSubSampling),
        531 => Some(Tags::YCbCrPositioning),
        282 => Some(Tags::XResolution),
        283 => Some(Tags::YResolution),
        296 => Some(Tags::ResolutionUnit),
        273 => Some(Tags::StripOffsets),
        278 => Some(Tags::RowsPerStrip),
        279 => Some(Tags::StripByteCounts),
        513 => Some(Tags::JPEGInterchangeFormat),
        514 => Some(Tags::JPEGInterchangeFormatLength),
        301 => Some(Tags::TransferFunction),
        318 => Some(Tags::WhitePoint),
        319 => Some(Tags::PrimaryChromaticities),
        529 => Some(Tags::YCbCrCoefficients),
        532 => Some(Tags::ReferenceBlackWhite),
        306 => Some(Tags::DateTime),
        270 => Some(Tags::ImageDescription),
        271 => Some(Tags::Make),
        272 => Some(Tags::Model),
        305 => Some(Tags::Software),
        315 => Some(Tags::Artist),
        33432 => Some(Tags::Copyright),
        // Exif
        36864 => Some(Tags::ExifVersion),
        40960 => Some(Tags::FlashpixVersion),
        40961 => Some(Tags::ColorSpace),
        42240 => Some(Tags::Gamma),
        37121 => Some(Tags::ComponentsConfiguration),
        37122 => Some(Tags::CompressedBitsPerPixel),
        40962 => Some(Tags::PixelXDimension),
        40963 => Some(Tags::PixelYDimension),
        37500 => Some(Tags::MakerNote),
        37510 => Some(Tags::UserComment),
        40964 => Some(Tags::RelatedSoundFile),
        36867 => Some(Tags::DateTimeOriginal),
        36868 => Some(Tags::DateTimeDigitized),
        36880 => Some(Tags::OffsetTime),
        36881 => Some(Tags::OffsetTimeOriginal),
        36882 => Some(Tags::OffsetTimeDigitized),
        37520 => Some(Tags::SubSecTime),
        37521 => Some(Tags::SubSecTimeOriginal),
        37522 => Some(Tags::SubSecTimeDigitized),
        33434 => Some(Tags::ExposureTime),
        33437 => Some(Tags::FNumber),
        34850 => Some(Tags::ExposureProgram),
        34852 => Some(Tags::SpectralSensitivity),
        34855 => Some(Tags::PhotographicSensitivity),
        34856 => Some(Tags::OECF),
        34864 => Some(Tags::SensitivityType),
        34865 => Some(Tags::StandardOutputSensitivity),
        34866 => Some(Tags::RecommendedExposureIndex),
        34867 => Some(Tags::ISOSpeed),
        34868 => Some(Tags::ISOSpeedLatitudeyyy),
        34869 => Some(Tags::ISOSpeedLatitudezzz),
        37377 => Some(Tags::ShutterSpeedValue),
        37378 => Some(Tags::ApertureValue),
        37379 => Some(Tags::BrightnessValue),
        37380 => Some(Tags::ExposureBiasValue),
        37381 => Some(Tags::MaxApertureValue),
        37382 => Some(Tags::SubjectDistance),
        37383 => Some(Tags::MeteringMode),
        37384 => Some(Tags::LightSource),
        37385 => Some(Tags::Flash),
        37386 => Some(Tags::FocalLength),
        37396 => Some(Tags::SubjectArea),
        41483 => Some(Tags::FlashEnergy),
        41484 => Some(Tags::SpatialFrequencyResponse),
        41486 => Some(Tags::FocalPlaneXResolution),
        41487 => Some(Tags::FocalPlaneYResolution),
        41488 => Some(Tags::FocalPlaneResolutionUnit),
        41492 => Some(Tags::SubjectLocation),
        41493 => Some(Tags::ExposureIndex),
        41495 => Some(Tags::SensingMethod),
        41728 => Some(Tags::FileSource),
        41729 => Some(Tags::SceneType),
        41730 => Some(Tags::CFAPattern),
        41985 => Some(Tags::CustomRendered),
        41986 => Some(Tags::ExposureMode),
        41987 => Some(Tags::WhiteBalance),
        41988 => Some(Tags::DigitalZoomRatio),
        41989 => Some(Tags::FocalLengthIn35mmFilm),
        41990 => Some(Tags::SceneCaptureType),
        41991 => Some(Tags::GainControl),
        41992 => Some(Tags::Contrast),
        41993 => Some(Tags::Saturation),
        41994 => Some(Tags::Sharpness),
        41995 => Some(Tags::DeviceSettingDescription),
        41996 => Some(Tags::SubjectDistanceRange),
        42080 => Some(Tags::CompositeImage),
        42081 => Some(Tags::SourceImageNumberOfCompositeImage),
        42082 => Some(Tags::SourceExposureTimesOfCompositeImage),
        37888 => Some(Tags::Temperature),
        37889 => Some(Tags::Humidity),
        37890 => Some(Tags::Pressure),
        37891 => Some(Tags::WaterDepth),
        37892 => Some(Tags::Acceleration),
        37893 => Some(Tags::CameraElevationAngle),
        42016 => Some(Tags::ImageUniqueID),
        42032 => Some(Tags::CameraOwnerName),
        42033 => Some(Tags::BodySerialNumber),
        42034 => Some(Tags::LensSpecification),
        42035 => Some(Tags::LensMake),
        42036 => Some(Tags::LensModel),
        42037 => Some(Tags::LensSerialNumber),
        // GPS
        //0 => Some(Tags::GPSVersionID),
        _ => None,
    }
}
