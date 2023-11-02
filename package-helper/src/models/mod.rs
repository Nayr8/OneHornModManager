
pub enum CompressionMethod {
    None,
    ZLib,
    LZ4,
    Invalid(u32),
}

impl From<u32> for CompressionMethod {
    fn from(value: u32) -> Self {
        match value {
            0 => CompressionMethod::None,
            1 => CompressionMethod::ZLib,
            2 => CompressionMethod::LZ4,
            _ => CompressionMethod::Invalid(value),
        }
    }
}

pub enum PackageVersion {
    DivinityOriginalSin,
    DivinityOriginalSinEnhancedEdition,
    DivinityOriginalSin2,
    DivinityOriginalSin2DefinitiveEdition,
    BaldursGate3EarlyAccess,
    BaldursGate3EarlyAccessPatch4,
    BaldursGate3,
    Invalid(u32),
}

impl From<u32> for PackageVersion {
    fn from(value: u32) -> Self {
        match value {
            7 => PackageVersion::DivinityOriginalSin,
            9 => PackageVersion::DivinityOriginalSinEnhancedEdition,
            10 => PackageVersion::DivinityOriginalSin2,
            13 => PackageVersion::DivinityOriginalSin2DefinitiveEdition,
            15 => PackageVersion::BaldursGate3EarlyAccess,
            16 => PackageVersion::BaldursGate3EarlyAccessPatch4,
            18 => PackageVersion::BaldursGate3,
            _ => PackageVersion::Invalid(value),
        }
    }
}