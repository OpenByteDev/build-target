use std::{
    env::{self, VarError},
    fmt,
};

use crate::utils::define_target_enum;

define_target_enum! {
    /// A more generic description of a target, such as the family of
    /// the operating systems or architectures that the target generally falls into.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Family<'a> {
        /// Unix based operating systems
        Unix => "unix",
        /// Microsoft's Windows operating system
        Windows => "windows",
        /// WebAssembly
        Wasm => "wasm",
    }

    as_str_doc = "String representing this target family which matches `#[cfg(target_family)]`",
    from_str_doc = "Tries to parse the given string as an [`Family`] falling back to [`Family::Other`] for unknown values.",
}

impl Family<'_> {
    /// Gets the current target [`Family`].
    pub fn target() -> Result<Vec<Self>, VarError> {
        env::var("CARGO_CFG_TARGET_FAMILY").map(|str| {
            str.split(',')
                .map(|s| Self::from_str(s.to_owned()))
                .collect()
        })
    }
}

impl fmt::Display for Family<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
