use crate::utils::{build_env_opt, define_target_enum};
use std::fmt;

define_target_enum! {
    // adapted from target/env.rs from platforms crate
    /// Target enviroment that disambiguates the target platform by ABI / libc.
    ///
    /// # Note
    /// This value is closely related to the fourth element of the platform target triple,
    /// though it is not identical. For example, embedded ABIs such as `gnueabihf` will simply
    /// define `target_env` as `"gnu"` (i.e. [`Env::Gnu`])
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Env {
        /// The GNU C Library (glibc)
        Gnu => "gnu",
        /// Microsoft Visual C(++)
        Msvc => "msvc",
        /// Clean, efficient, standards-conformant libc implementation.
        Musl => "musl",
        /// Newlib environment
        Newlib => "newlib",
        /// NTO 7.0 environment
        Nto70 => "nto70",
        /// NTO 7.1 environment
        Nto71 => "nto71",
        /// NTO 7.1 iOSOCK environment
        Nto71Iosock => "nto71_iosock",
        /// NTO 8.0 environment
        Nto80 => "nto80",
        /// Open Harmony OS
        OhOS => "ohos",
        P1 => "p1",
        P2 => "p2",
        /// Relibc environment
        Relibc => "relibc",
        /// Intel Software Guard Extensions (SGX) Enclave
        Sgx => "sgx",
        /// C library for developing embedded Linux systems
        UClibc => "uclibc"
    }

    as_str_doc = "String representing this environment which matches `#[cfg(target_env)]`.",
    from_str_doc = "Tries to parse the given string as an [`Env`] falling back to [`Env::Other`] for unknown values.",
}

impl Env {
    /// Gets the current target [`Env`].
    #[must_use]
    pub fn target() -> Option<Self> {
        build_env_opt("CARGO_CFG_TARGET_ENV").map(Self::from_str)
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
