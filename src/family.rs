use std::{
    borrow::Cow,
    env::{self, VarError},
    fmt,
};

/// `target_family`: A a more generic description of a target, such as the family of
/// the operating systems or architectures that the target generally falls into.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Family<'a> {
    /// `unix`: Unix based operating systems
    Unix,

    /// `windows`: Microsoft's Windows operating system
    Windows,

    /// `wasm`: WebAssembly
    Wasm,

    /// Operating system family we don't know about
    Other(Cow<'a, str>),
}

impl<'a> Family<'a> {
    /// String representing this target family which matches `#[cfg(target_family)]`
    pub fn as_str(&self) -> &str {
        match self {
            Family::Unix => "unix",
            Family::Windows => "windows",
            Family::Wasm => "wasm",
            Family::Other(s) => s,
        }
    }

    /// Create a new [`Arch`] from the given string.
    fn from_str(os_name: impl Into<Cow<'a, str>>) -> Self {
        let os_name = os_name.into();
        match os_name.as_ref() {
            "unix" => Family::Unix,
            "windows" => Family::Windows,
            "wasm" => Family::Wasm,
            _ => Family::Other(os_name),
        }
    }

    pub fn target() -> Result<Self, VarError> {
        env::var("CARGO_CFG_TARGET_FAMILY").map(Self::from_str)
    }
}

impl<'a> fmt::Display for Family<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
