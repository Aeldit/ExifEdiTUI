use core::fmt;

#[derive(PartialEq)]
pub struct Tag(pub usize);

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Tags {
    use super::Tag;

    // Special IFDs
    pub const ExifOffset: Tag = Tag(34665);
    pub const GPSOffset: Tag = Tag(34853);
    pub const InteroperabilityIFD: Tag = Tag(40965);

    // TIFF
    // Image Data Structure
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

    // Offset Recording
    pub const StripOffsets: Tag = Tag(273);
    pub const RowsPerStrip: Tag = Tag(278);
    pub const StripByteCounts: Tag = Tag(279);
    pub const JPEGInterchangeFormat: Tag = Tag(513);
    pub const JPEGInterchangeFormatLength: Tag = Tag(514);

    // Image Data Characteristics
    pub const TransferFunction: Tag = Tag(301);
    pub const WhitePoint: Tag = Tag(318);
    pub const PrimaryChromaticities: Tag = Tag(319);
    pub const YCbCrCoefficients: Tag = Tag(529);
    pub const ReferenceBlackWhite: Tag = Tag(532);

    // Other
    pub const DateTime: Tag = Tag(306);
    pub const ImageDescription: Tag = Tag(270);
    pub const Make: Tag = Tag(271);
    pub const Model: Tag = Tag(272);
    pub const Software: Tag = Tag(305);
    pub const Artist: Tag = Tag(315);
    pub const Copyright: Tag = Tag(33432);

    ////////////////////////////////////////////////////////////////////////////
    // Exif
    ////////////////////////////////////////////////////////////////////////////
    // Tags Relating to Version
    pub const ExifVersion: Tag = Tag(36864);
    pub const FlashpixVersion: Tag = Tag(40960);

    // Tags Relating to ColorSpace
    pub const ColorSpace: Tag = Tag(40961);
    pub const Gamma: Tag = Tag(42240);

    // Tags Relating to Image Configuration
    pub const ComponentsConfiguration: Tag = Tag(37121);
    pub const CompressedBitsPerPixel: Tag = Tag(37122);
    pub const PixelXDimension: Tag = Tag(40962);
    pub const PixelYDimension: Tag = Tag(40963);

    // Tags Relating to User Information
    pub const MakerNote: Tag = Tag(37500);
    pub const UserComment: Tag = Tag(37510);

    // Tag Relating to Related File Information
    pub const RelatedSoundFile: Tag = Tag(40964);

    // Tags Relating to Date and Time
    pub const DateTimeOriginal: Tag = Tag(36867);
    pub const DateTimeDigitized: Tag = Tag(36868);
    pub const OffsetTime: Tag = Tag(36880);
    pub const OffsetTimeOriginal: Tag = Tag(36881);
    pub const OffsetTimeDigitized: Tag = Tag(36882);
    pub const SubSecTime: Tag = Tag(37520);
    pub const SubSecTimeOriginal: Tag = Tag(37521);
    pub const SubSecTimeDigitized: Tag = Tag(37522);

    // Tags Relating to Picture-Taking Conditions
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

    // Tags Relating to shooting situation
    pub const Temperature: Tag = Tag(37888);
    pub const Humidity: Tag = Tag(37889);
    pub const Pressure: Tag = Tag(37890);
    pub const WaterDepth: Tag = Tag(37891);
    pub const Acceleration: Tag = Tag(37892);
    pub const CameraElevationAngle: Tag = Tag(37893);

    // Other Tags
    pub const ImageUniqueID: Tag = Tag(42016);
    pub const CameraOwnerName: Tag = Tag(42032);
    pub const BodySerialNumber: Tag = Tag(42033);
    pub const LensSpecification: Tag = Tag(42034);
    pub const LensMake: Tag = Tag(42035);
    pub const LensModel: Tag = Tag(42036);
    pub const LensSerialNumber: Tag = Tag(42037);

    ////////////////////////////////////////////////////////////////////////////
    // GPS
    ////////////////////////////////////////////////////////////////////////////
    pub const GPSVersionID: Tag = Tag(0);
    pub const GPSLatitudeRef: Tag = Tag(1);
    pub const GPSLatitude: Tag = Tag(2);
    pub const GPSLongitudeRef: Tag = Tag(3);
    pub const GPSLongitude: Tag = Tag(4);
    pub const GPSAltitudeRef: Tag = Tag(5);
    pub const GPSAltitude: Tag = Tag(6);
    pub const GPSTimeStamp: Tag = Tag(7);
    pub const GPSSatellites: Tag = Tag(8);
    pub const GPSStatus: Tag = Tag(9);
    pub const GPSMeasureMode: Tag = Tag(10);
    pub const GPSDOP: Tag = Tag(11);
    pub const GPSSpeedRef: Tag = Tag(12);
    pub const GPSSpeed: Tag = Tag(13);
    pub const GPSTrackRef: Tag = Tag(14);
    pub const GPSTrack: Tag = Tag(15);
    pub const GPSImgDirectionRef: Tag = Tag(16);
    pub const GPSImgDirection: Tag = Tag(17);
    pub const GPSMapDatum: Tag = Tag(18);
    pub const GPSDestLatitudeRef: Tag = Tag(19);
    pub const GPSDestLatitude: Tag = Tag(20);
    pub const GPSDestLongitudeRef: Tag = Tag(21);
    pub const GPSDestLongitude: Tag = Tag(22);
    pub const GPSDestBearingRef: Tag = Tag(23);
    pub const GPSDestBearing: Tag = Tag(24);
    pub const GPSDestDistanceRef: Tag = Tag(25);
    pub const GPSDestDistance: Tag = Tag(26);
    pub const GPSProcessingMethod: Tag = Tag(27);
    pub const GPSAreaInformation: Tag = Tag(28);
    pub const GPSDateStamp: Tag = Tag(29);
    pub const GPSDifferential: Tag = Tag(30);
    pub const GPSHPositioningError: Tag = Tag(31);
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tags::ExifOffset => "ExifOffset",
                Tags::GPSOffset => "GPSOffset",
                Tags::InteroperabilityIFD => "InteroperabilityIFD",
                Tags::ImageWidth => "ImageWidth",
                Tags::ImageLength => "ImageLength",
                Tags::BitsPerSample => "BitsPerSample",
                Tags::Compression => "Compression",
                Tags::PhotometricInterpretation => "PhotometricInterpretation",
                Tags::Orientation => "Orientation",
                Tags::SamplesPerPixel => "SamplesPerPixel",
                Tags::PlanarConfiguration => "PlanarConfiguration",
                Tags::YCbCrSubSampling => "YCbCrSubSampling",
                Tags::YCbCrPositioning => "YCbCrPositioning",
                Tags::XResolution => "XResolution",
                Tags::YResolution => "YResolution",
                Tags::ResolutionUnit => "ResolutionUnit",
                Tags::StripOffsets => "StripOffsets",
                Tags::RowsPerStrip => "RowsPerStrip",
                Tags::StripByteCounts => "StripByteCounts",
                Tags::JPEGInterchangeFormat => "JPEGInterchangeFormat",
                Tags::JPEGInterchangeFormatLength => "JPEGInterchangeFormatLength",
                Tags::TransferFunction => "TransferFunction",
                Tags::WhitePoint => "WhitePoint",
                Tags::PrimaryChromaticities => "PrimaryChromaticities",
                Tags::YCbCrCoefficients => "YCbCrCoefficients",
                Tags::ReferenceBlackWhite => "ReferenceBlackWhite",
                Tags::DateTime => "DateTime",
                Tags::ImageDescription => "ImageDescription",
                Tags::Make => "Make",
                Tags::Model => "Model",
                Tags::Software => "Software",
                Tags::Artist => "Artist",
                Tags::Copyright => "Copyright",
                Tags::ExifVersion => "ExifVersion",
                Tags::FlashpixVersion => "FlashpixVersion",
                Tags::ColorSpace => "ColorSpace",
                Tags::Gamma => "Gamma",
                Tags::ComponentsConfiguration => "ComponentsConfiguration",
                Tags::CompressedBitsPerPixel => "CompressedBitsPerPixel",
                Tags::PixelXDimension => "PixelXDimension",
                Tags::PixelYDimension => "PixelYDimension",
                Tags::MakerNote => "MakerNote",
                Tags::UserComment => "UserComment",
                Tags::RelatedSoundFile => "RelatedSoundFile",
                Tags::DateTimeOriginal => "DateTimeOriginal",
                Tags::DateTimeDigitized => "DateTimeDigitized",
                Tags::OffsetTime => "OffsetTime",
                Tags::OffsetTimeOriginal => "OffsetTimeOriginal",
                Tags::OffsetTimeDigitized => "OffsetTimeDigitized",
                Tags::SubSecTime => "SubSecTime",
                Tags::SubSecTimeOriginal => "SubSecTimeOriginal",
                Tags::SubSecTimeDigitized => "SubSecTimeDigitized",
                Tags::ExposureTime => "ExposureTime",
                Tags::FNumber => "FNumber",
                Tags::ExposureProgram => "ExposureProgram",
                Tags::SpectralSensitivity => "SpectralSensitivity",
                Tags::PhotographicSensitivity => "PhotographicSensitivity",
                Tags::OECF => "OECF",
                Tags::SensitivityType => "SensitivityType",
                Tags::StandardOutputSensitivity => "StandardOutputSensitivity",
                Tags::RecommendedExposureIndex => "RecommendedExposureIndex",
                Tags::ISOSpeed => "ISOSpeed",
                Tags::ISOSpeedLatitudeyyy => "ISOSpeedLatitudeyyy",
                Tags::ISOSpeedLatitudezzz => "ISOSpeedLatitudezzz",
                Tags::ShutterSpeedValue => "ShutterSpeedValue",
                Tags::ApertureValue => "ApertureValue",
                Tags::BrightnessValue => "BrightnessValue",
                Tags::ExposureBiasValue => "ExposureBiasValue",
                Tags::MaxApertureValue => "MaxApertureValue",
                Tags::SubjectDistance => "SubjectDistance",
                Tags::MeteringMode => "MeteringMode",
                Tags::LightSource => "LightSource",
                Tags::Flash => "Flash",
                Tags::FocalLength => "FocalLength",
                Tags::SubjectArea => "SubjectArea",
                Tags::FlashEnergy => "FlashEnergy",
                Tags::SpatialFrequencyResponse => "SpatialFrequencyResponse",
                Tags::FocalPlaneXResolution => "FocalPlaneXResolution",
                Tags::FocalPlaneYResolution => "FocalPlaneYResolution",
                Tags::FocalPlaneResolutionUnit => "FocalPlaneResolutionUnit",
                Tags::SubjectLocation => "SubjectLocation",
                Tags::ExposureIndex => "ExposureIndex",
                Tags::SensingMethod => "SensingMethod",
                Tags::FileSource => "FileSource",
                Tags::SceneType => "SceneType",
                Tags::CFAPattern => "CFAPattern",
                Tags::CustomRendered => "CustomRendered",
                Tags::ExposureMode => "ExposureMode",
                Tags::WhiteBalance => "WhiteBalance",
                Tags::DigitalZoomRatio => "DigitalZoomRatio",
                Tags::FocalLengthIn35mmFilm => "FocalLengthIn35mmFilm",
                Tags::SceneCaptureType => "SceneCaptureType",
                Tags::GainControl => "GainControl",
                Tags::Contrast => "Contrast",
                Tags::Saturation => "Saturation",
                Tags::Sharpness => "Sharpness",
                Tags::DeviceSettingDescription => "DeviceSettingDescription",
                Tags::SubjectDistanceRange => "SubjectDistanceRange",
                Tags::CompositeImage => "CompositeImage",
                Tags::SourceImageNumberOfCompositeImage => "SourceImageNumberOfCompositeImage",
                Tags::SourceExposureTimesOfCompositeImage => "SourceExposureTimesOfCompositeImage",
                Tags::Temperature => "Temperature",
                Tags::Humidity => "Humidity",
                Tags::Pressure => "Pressure",
                Tags::WaterDepth => "WaterDepth",
                Tags::Acceleration => "Acceleration",
                Tags::CameraElevationAngle => "CameraElevationAngle",
                Tags::ImageUniqueID => "ImageUniqueID",
                Tags::CameraOwnerName => "CameraOwnerName",
                Tags::BodySerialNumber => "BodySerialNumber",
                Tags::LensSpecification => "LensSpecification",
                Tags::LensMake => "LensMake",
                Tags::LensModel => "LensModel",
                Tags::LensSerialNumber => "LensSerialNumber",
                Tags::GPSVersionID => "GPSVersionID",
                Tags::GPSLatitudeRef => "GPSLatitudeRef",
                Tags::GPSLatitude => "GPSLatitude",
                Tags::GPSLongitudeRef => "GPSLongitudeRef",
                Tags::GPSLongitude => "GPSLongitude",
                Tags::GPSAltitudeRef => "GPSAltitudeRef",
                Tags::GPSAltitude => "GPSAltitude",
                Tags::GPSTimeStamp => "GPSTimeStamp",
                Tags::GPSSatellites => "GPSSatellites",
                Tags::GPSStatus => "GPSStatus",
                Tags::GPSMeasureMode => "GPSMeasureMode",
                Tags::GPSDOP => "GPSDOP",
                Tags::GPSSpeedRef => "GPSSpeedRef",
                Tags::GPSSpeed => "GPSSpeed",
                Tags::GPSTrackRef => "GPSTrackRef",
                Tags::GPSTrack => "GPSTrack",
                Tags::GPSImgDirectionRef => "GPSImgDirectionRef",
                Tags::GPSImgDirection => "GPSImgDirection",
                Tags::GPSMapDatum => "GPSMapDatum",
                Tags::GPSDestLatitudeRef => "GPSDestLatitudeRef",
                Tags::GPSDestLatitude => "GPSDestLatitude",
                Tags::GPSDestLongitudeRef => "GPSDestLongitudeRef",
                Tags::GPSDestLongitude => "GPSDestLongitude",
                Tags::GPSDestBearingRef => "GPSDestBearingRef",
                Tags::GPSDestBearing => "GPSDestBearing",
                Tags::GPSDestDistanceRef => "GPSDestDistanceRef",
                Tags::GPSDestDistance => "GPSDestDistance",
                Tags::GPSProcessingMethod => "GPSProcessingMethod",
                Tags::GPSAreaInformation => "GPSAreaInformation",
                Tags::GPSDateStamp => "GPSDateStamp",
                Tags::GPSDifferential => "GPSDifferential",
                Tags::GPSHPositioningError => "GPSHPositioningError",
                _ => "unknown",
            }
        )
    }
}
