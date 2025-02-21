use core::fmt;

use crate::arrays::{get_tuples_vec_as_string, get_vec_as_string};

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

pub fn get_short_string_for_tag(tag: Tag, count: usize, values: Vec<u16>) -> String {
    if tag == Tags::Compression && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 1 {
                "uncompressed"
            } else if values[0] == 6 {
                "JPEG compression (thumbnails only)"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::PhotometricInterpretation && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 2 {
                "RGB"
            } else if values[0] == 6 {
                "YCbCr"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::PlanarConfiguration && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 1 {
                "chunky format"
            } else if values[0] == 2 {
                "planar format"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::YCbCrSubSampling && count == 2 && values.len() == 2 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 2 && values[1] == 1 {
                "YCbCr4:2:2"
            } else if values[0] == 2 && values[1] == 2 {
                "YCbCr4:2:0"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::YCbCrPositioning && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 1 {
                "centered"
            } else if values[0] == 2 {
                "co-sited"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::ResolutionUnit && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 2 {
                "inches"
            } else if values[0] == 3 {
                "centimeters"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::ColorSpace && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            if values[0] == 1 {
                "sRGB"
            } else if values[0] == 0xFFFF {
                "Uncalibrated"
            } else {
                "reserved"
            }
        )
    } else if tag == Tags::ExposureProgram && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Not defined",
                1 => "Manual",
                2 => "Normal program",
                3 => "Aperture priority",
                4 => "Shutter priority",
                5 => "Creative program (biased toward depth of field)",
                6 => "Action program (biased toward fast shutter speed)",
                7 => "Portrait mode (for closeup photos with the background out of focus)",
                8 => "Landscape mode (for landscape photos with the background in focus)",
                _ => "reserved",
            }
        )
    } else if tag == Tags::SensitivityType && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Unknown",
                1 => "Standard output sensitivity (SOS)",
                2 => "Recommended exposure index (REI)",
                3 => "ISO speed",
                4 => "Standard output sensitivity (SOS) and recommended exposure index (REI)",
                5 => "Standard output sensitivity (SOS) and ISO speed",
                6 => "Recommended exposure index (REI) and ISO speed",
                7 => "Standard output sensitivity (SOS) and recommended exposure index (REI) and ISO speed",
                _ => "reserved",
            }
        )
    } else if tag == Tags::MeteringMode && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "unknown",
                1 => "Average",
                2 => "CenterWeightedAverage",
                3 => "Spot",
                4 => "MultiSpot",
                5 => "Pattern",
                6 => "Partial",
                255 => "other",
                _ => "reserved",
            }
        )
    } else if tag == Tags::LightSource && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "unknown",
                1 => "Daylight",
                2 => "Fluorescent",
                3 => "Tungsten (incandescent light)",
                4 => "Flash",
                9 => "Fine weather",
                10 => "Cloudy weather",
                11 => "Shade",
                12 => "Daylight fluorescent (D 5700 - 7100K)",
                13 => "Day white fluorescent (N 4600 - 5500K)",
                14 => "Cool white fluorescent (W 3800 - 4500K)",
                15 => "White fluorescent (WW 3250 - 3800K)",
                16 => "Warm white fluorescent (L 2600 - 3250K)",
                17 => "Standard light A",
                18 => "Standard light B",
                19 => "Standard light C",
                20 => "D55",
                21 => "D65",
                22 => "D75",
                23 => "D50",
                24 => "ISO studio tungsten",
                255 => "other light source",
                _ => "reserved",
            }
        )
    } else if tag == Tags::Flash && count == 1 && values.len() == 1 {
        let v = values[0];
        format!(
            "{}: {} / {} / {} / {} / {}",
            tag,
            if v & (1 << 0) == (1 << 0) {
                "Flash fired"
            } else {
                "Flash did not fire"
            },
            if v & 2 == 0 && v & 3 == 0 {
                "No strobe return detection function"
            } else if v & (1 << 1) == 0 && v & (1 << 2) == (1 << 2) {
                "reserved"
            } else if v & (1 << 1) == (1 << 1) && v & (1 << 2) == 0 {
                "Strobe return light not detected"
            } else {
                "Strobe return light detected"
            },
            if v & (1 << 3) == 0 && v & (1 << 4) == 0 {
                "unknown"
            } else if v & (1 << 3) == 0 && v & (1 << 4) == (1 << 4) {
                "Compulsory flash firing"
            } else if v & (1 << 3) == (1 << 3) && v & (1 << 4) == 0 {
                "Compulsory flash suppression"
            } else {
                "Auto mode"
            },
            if v & (1 << 5) == (1 << 5) {
                "No flash function"
            } else {
                "Flash function present"
            },
            if v & (1 << 6) == (1 << 6) {
                "Red-eye reduction supported"
            } else {
                "No red-eye reduction mode or unknown"
            },
        )
    } else if tag == Tags::SensingMethod && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                1 => "Not defined",
                2 => "One-chip color area sensor",
                3 => "Two-chip color area sensor",
                4 => "Three-chip color area sensor",
                5 => "Color sequential area sensor",
                7 => "Trilinear sensor",
                8 => "Color sequential linear sensor",
                _ => "reserved",
            }
        )
    } else if tag == Tags::CustomRendered && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Normal process",
                1 => "Custom process",
                _ => "reserved",
            }
        )
    } else if tag == Tags::ExposureMode && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Auto exposure",
                1 => "Manual exposure",
                2 => "Auto bracket",
                _ => "reserved",
            }
        )
    } else if tag == Tags::WhiteBalance && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Auto white balance",
                1 => "Manual white balance",
                _ => "reserved",
            }
        )
    } else if tag == Tags::SceneCaptureType && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Standard",
                1 => "Landscape",
                2 => "Portrait",
                3 => "Night scene",
                _ => "reserved",
            }
        )
    } else if tag == Tags::GainControl && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "None",
                1 => "Low gain up",
                2 => "High gain up",
                3 => "Low gain down",
                4 => "High gain down",
                _ => "reserved",
            }
        )
    } else if tag == Tags::Contrast && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Normal",
                1 => "Soft",
                2 => "Hard",
                _ => "reserved",
            }
        )
    } else if tag == Tags::Saturation && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Normal",
                1 => "Low saturation",
                2 => "High saturation",
                _ => "reserved",
            }
        )
    } else if tag == Tags::Sharpness && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "Normal",
                1 => "Soft",
                2 => "Hard",
                _ => "reserved",
            }
        )
    } else if tag == Tags::SubjectDistanceRange && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "unknown",
                1 => "Macro",
                2 => "Close view",
                3 => "Distant view",
                _ => "reserved",
            }
        )
    } else if tag == Tags::CompositeImage && count == 1 && values.len() == 1 {
        format!(
            "{}: {}",
            tag,
            match values[0] {
                0 => "unknown",
                1 => "non-composite image",
                2 => "General composite image",
                3 => "Composite image captured when shooting",
                _ => "reserved",
            }
        )
    } else {
        format!("{}: {}", tag, get_vec_as_string(values))
    }
}

pub fn get_rational_string_for_tag(tag: Tag, count: usize, values: Vec<(u32, u32)>) -> String {
    if tag == Tags::LensSpecification && count == 4 && values.len() == 4 {
        format!("{}: {}", tag, "TODO")
    } else {
        format!("{}: {}", tag, get_tuples_vec_as_string(values))
    }
}

pub fn get_undefined_string_for_tag(tag: Tag, count: usize, value_offset: [u8; 4]) -> String {
    if tag == Tags::ExifVersion && count == 4 {
        format!(
            "{}: {:?}",
            tag,
            String::from_iter(value_offset.iter().map(|b| *b as char))
        )
    } else if tag == Tags::FlashpixVersion
        && count == 4
        && value_offset[0] == 48
        && value_offset[1] == 49
        && value_offset[2] == 48
        && value_offset[3] == 48
    {
        format!("{}: Flashpix Format Version 1.0", tag)
    } else if tag == Tags::ComponentsConfiguration && count == 4 {
        format!(
            "{}: {}{}{}",
            tag,
            match value_offset[0] {
                1 => "Y",
                2 => "Cb",
                3 => "Cr",
                4 => "R",
                5 => "G",
                6 => "B",
                _ => "",
            },
            match value_offset[1] {
                1 => "Y",
                2 => "Cb",
                3 => "Cr",
                4 => "R",
                5 => "G",
                6 => "B",
                _ => "",
            },
            match value_offset[2] {
                1 => "Y",
                2 => "Cb",
                3 => "Cr",
                4 => "R",
                5 => "G",
                6 => "B",
                _ => "",
            }
        )
    } else if tag == Tags::OECF {
        todo!() // TODO:
    } else if tag == Tags::FileSource && count == 1 {
        format!(
            "{}: {}",
            tag,
            match value_offset[0] {
                0 => "others",
                1 => "scanner of transparent type",
                2 => "scanner of reflex type",
                3 => "DSC",
                _ => "reserved",
            }
        )
    } else if tag == Tags::SceneType && count == 1 {
        format!(
            "{}: {}",
            tag,
            if value_offset[0] == 1 {
                "A directly photographed image"
            } else {
                "reserved"
            }
        )
    } else {
        format!("{}: Undefined", tag)
    }
}
