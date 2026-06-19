// This is free and unencumbered software released into the public domain.

/// A base direction for directional language-tagged strings.
///
/// See: <https://www.w3.org/TR/rdf12-concepts/#dfn-base-direction>
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaseDirection {
    /// The initial text direction is set to left-to-right (LTR)
    #[default]
    Ltr,

    /// The initial text direction is set to right-to-left (RTL)
    Rtl,
}

impl BaseDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

#[cfg(feature = "oxrdf")]
impl From<oxrdf::BaseDirection> for BaseDirection {
    fn from(input: oxrdf::BaseDirection) -> Self {
        use oxrdf::BaseDirection::*;
        match input {
            Ltr => Self::Ltr,
            Rtl => Self::Rtl,
        }
    }
}

#[cfg(feature = "oxrdf")]
impl From<BaseDirection> for oxrdf::BaseDirection {
    fn from(input: BaseDirection) -> Self {
        use BaseDirection::*;
        match input {
            Ltr => Self::Ltr,
            Rtl => Self::Rtl,
        }
    }
}
