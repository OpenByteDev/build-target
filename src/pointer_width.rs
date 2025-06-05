use std::{
    env::{self, VarError},
    fmt,
    num::ParseIntError,
};

use crate::utils::define_target_enum;

define_target_enum! {
    /// The endianness of the target architecture.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum PointerWidth<'a> {
        /// 16-bit pointer width.
        U16 => "16",
        /// 32-bit pointer width.
        U32 => "32",
        /// 64-bit pointer width.
        U64 => "64",
    }

    as_str_doc = "String representing this target pointer width which matches `#[cfg(target_pointer_width)]`",
    from_str_doc = "Tries to parse the given string as a [`PointerWidth`] falling back to [`PointerWidth::Other`] for unknown values.",
}

impl PointerWidth<'_> {
    /// Gets the current target [`PointerWidth`].
    pub fn target() -> Result<Self, VarError> {
        env::var("CARGO_CFG_TARGET_POINTER_WIDTH").map(Self::from_str)
    }
}

impl fmt::Display for PointerWidth<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<u8> for PointerWidth<'_> {
    fn from(value: u8) -> Self {
        match value {
            64 => PointerWidth::U64,
            32 => PointerWidth::U32,
            16 => PointerWidth::U16,
            _ => PointerWidth::Other(value.to_string().into()),
        }
    }
}

impl TryFrom<PointerWidth<'_>> for u8 {
    type Error = ParseIntError;

    fn try_from(value: PointerWidth<'_>) -> Result<Self, Self::Error> {
        match value {
            PointerWidth::U64 => Ok(64),
            PointerWidth::U32 => Ok(32),
            PointerWidth::U16 => Ok(16),
            PointerWidth::Other(s) => s.parse(),
        }
    }
}
