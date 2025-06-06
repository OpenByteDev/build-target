use std::{
    env::{self, VarError},
    fmt,
};

use crate::utils::define_target_enum;

define_target_enum! {
    /// The endianness of the target architecture.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Endian<'a> {
        /// Most significant byte stored first.
        Big => "big",
        /// Least significant byte stored first.
        Little => "little",
    }

    as_str_doc = "String representing this target endianness which matches `#[cfg(target_endian)]`",
    from_str_doc = "Tries to parse the given string as an [`Endian`] falling back to [`Endian::Other`] for unknown values.",
}

impl Endian<'_> {
    /// Gets the current target [`Endian`].
    pub fn target() -> Result<Self, VarError> {
        env::var("CARGO_CFG_TARGET_ENDIAN").map(Self::from_str)
    }
}

impl fmt::Display for Endian<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
