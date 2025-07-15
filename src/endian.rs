use std::fmt;

use crate::utils::{build_env, define_target_enum};

define_target_enum! {
    /// The endianness of the target architecture.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Endian {
        /// Most significant byte stored first.
        Big => "big",
        /// Least significant byte stored first.
        Little => "little",
    }

    as_str_doc = "String representing this target endianness which matches `#[cfg(target_endian)]`",
    from_str_doc = "Tries to parse the given string as an [`Endian`] falling back to [`Endian::Other`] for unknown values.",
}

impl Endian {
    /// Gets the current target [`Endian`].
    #[must_use]
    pub fn target() -> Self {
        Self::from_str(build_env("CARGO_CFG_TARGET_ENDIAN"))
    }
}

impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
