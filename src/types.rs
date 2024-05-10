use std::fmt::Display;

#[derive(Debug)]
pub struct Image {
    pub id: u64,
    pub title: String,
    pub source_url: String,
    pub image_url: String,
    pub licence: Licence,
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Unposted = 0,
    Success = 1,
    DownloadFail = 2,
    ImageTooLarge = 3,
    PostFail = 4,
}

// Possible licences as of 2022-11-13
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Licence {
    AllRightsReserved,
    CC_BY_SA_NC,
    CC_BY_NC,
    CC_BY_NC_ND,
    CC_BY,
    CC_BY_SA,
    CC_BY_ND,
    NoKnownRestrictions,
    USGov,
    CC0,
    PublicDomain,
    Unknown,
}

impl From<String> for Licence {
    fn from(value: String) -> Self {
        match value.as_str() {
            "All Rights Reserved" => Self::AllRightsReserved,
            "Attribution-NonCommercial-ShareAlike License" => Self::CC_BY_SA_NC,
            "Attribution-NonCommercial License" => Self::CC_BY_NC,
            "Attribution-NonCommercial-NoDerivs License" => Self::CC_BY_NC_ND,
            "Attribution License" => Self::CC_BY,
            "Attribution-ShareAlike License" => Self::CC_BY_SA,
            "Attribution-NoDerivs License" => Self::CC_BY_ND,
            "No known copyright restrictions" => Self::NoKnownRestrictions,
            "United States Government Work" => Self::USGov,
            "Public Domain Dedication (CC0)" => Self::CC0,
            "Public Domain Mark" => Self::PublicDomain,
            _ => Self::Unknown
        }
    }
}

impl Display for Licence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::AllRightsReserved => "All Rights Reserved",
            Self::CC_BY_SA_NC => "Creative Commons BY-SA-NC",
            Self::CC_BY_NC => "Creative Commons BY-NC",
            Self::CC_BY_NC_ND => "Creative Commons BY-NC-ND",
            Self::CC_BY => "Creative Commons BY",
            Self::CC_BY_SA => "Creative Commons BY-SA",
            Self::CC_BY_ND => "Creative Commons BY-ND",
            Self::NoKnownRestrictions => "No known copyright restrictions",
            Self::USGov => "U.S. Goverment Work",
            Self::CC0 => "Creative Commons 0",
            Self::PublicDomain => "Public Domain",
            Self::Unknown => "Unknown Licence",
        })
    }
}
