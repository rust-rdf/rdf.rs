// This is free and unencumbered software released into the public domain.

/// A slot in a statement (subject, predicate, object, or context).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementSlot {
    /// The subject slot
    Subject,

    /// The predicate slot
    Predicate,

    /// The object slot
    Object,

    /// The context (or graph) slot
    Context,
}

impl core::fmt::Display for StatementSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl StatementSlot {
    pub fn as_char(&self) -> char {
        match self {
            Self::Subject => 's',
            Self::Predicate => 'p',
            Self::Object => 'o',
            Self::Context => 'c', // aka 'g'
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Predicate => "predicate",
            Self::Object => "object",
            Self::Context => "context",
        }
    }
}
