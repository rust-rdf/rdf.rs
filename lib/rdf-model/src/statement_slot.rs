// This is free and unencumbered software released into the public domain.

pub const TRIPLE_SLOTS: &[StatementSlot] = &[
    StatementSlot::Subject,
    StatementSlot::Predicate,
    StatementSlot::Object,
];

pub const QUAD_SLOTS: &[StatementSlot] = &[
    StatementSlot::Subject,
    StatementSlot::Predicate,
    StatementSlot::Object,
    StatementSlot::Context,
];

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
    pub fn index(&self) -> usize {
        self.as_usize()
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Self::Subject => 0,
            Self::Predicate => 1,
            Self::Object => 2,
            Self::Context => 3,
        }
    }

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
