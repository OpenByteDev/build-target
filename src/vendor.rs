use std::fmt;

use crate::utils::{build_env, define_target_enum};

define_target_enum! {
    /// The vendor of the target platform, such as the manufacturer of the hardware or the provider of the operating system.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Vendor {
        /// Apple Inc.
        Apple => "apple",
        /// Fortanix SGX platform
        Fortanix => "fortanix",
        /// NVIDIA Corporation
        Nvidia => "nvidia",
        /// Generic PC platform
        Pc => "pc",
        /// Sony platform (e.g., PlayStation)
        Sony => "sony",
        /// Unknown or unspecified vendor
        Unknown => "unknown",
        /// Wind River Systems
        Wrs => "wrs",
        /// Universal Windows Platform
        Uwp => "uwp",
    }

    as_str_doc = "String representing this target vendor which matches `#[cfg(target_vendor)]`",
    from_str_doc = "Tries to parse the given string as an [`Vendor`] falling back to [`Vendor::Other`] for unknown values.",
}

impl Vendor {
    /// Gets the current target [`Vendor`].
    #[must_use]
    pub fn target() -> Self {
        Self::from_str(build_env("CARGO_CFG_TARGET_VENDOR"))
    }
}

impl fmt::Display for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
