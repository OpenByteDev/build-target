use std::fmt;

use crate::utils::{build_env, define_target_enum};

define_target_enum! {
    // adapted from target/os.rs from platforms crate
    /// Operating system of the target.
    ///
    /// # Note
    /// This value is closely related to the second
    /// and third element of the platform target triple, though it is not identical.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Os {
        /// IBM AIX operating system
        Aix => "aix",
        /// AMD HSA architecture
        Amdhsa => "amdhsa",
        /// Google’s Android mobile operating system
        Android => "android",
        /// CUDA parallel computing platform
        Cuda => "cuda",
        /// POSIX layer for Windows
        Cygwin => "cygwin",
        /// DragonflyBSD
        Dragonfly => "dragonfly",
        /// The emscripten JavaScript transpiler
        Emscripten => "emscripten",
        /// Espressif IoT framework
        Espidf => "espidf",
        /// The FreeBSD operating system
        FreeBSD => "freebsd",
        /// Google’s next-gen Rust OS
        Fuchsia => "fuchsia",
        /// Haiku, an open source BeOS clone
        Haiku => "haiku",
        /// Unikernel targeting HPC and cloud environments
        Hermit => "hermit",
        Horizon => "horizon",
        Hurd => "hurd",
        /// illumos is a partly free and open-source Unix OS based on OpenSolaris
        IllumOS => "illumos",
        /// Apple’s iOS mobile operating system
        #[allow(non_camel_case_types)]
        iOS => "ios",
        /// Microkernel OS framework
        L4re => "l4re",
        /// Linux
        Linux => "linux",
        /// Real-time OS
        Lynxos178 => "lynxos178",
        /// Apple’s Mac OS X
        MacOS => "macos",
        /// The NetBSD operating system
        NetBSD => "netbsd",
        None => "none",
        /// QNX Neutrino OS
        Nto => "nto",
        /// Embedded real-time OS
        Nuttx => "nuttx",
        /// The OpenBSD operating system
        OpenBSD => "openbsd",
        /// PlayStation Portable OS
        Psp => "psp",
        /// PlayStation OS
        Psx => "psx",
        /// Redox, a Unix-like OS written in Rust
        Redox => "redox",
        /// Real-time executive OS
        Rtems => "rtems",
        /// Oracle’s (formerly Sun) Solaris operating system
        Solaris => "solaris",
        SolidAsp3 => "solid_asp3",
        /// Trusted Execution Environment OS
        TeeOS => "teeos",
        /// Android trusted environment
        Trusty => "trusty",
        /// Apple TV OS
        TvOS => "tvos",
        /// Firmware interface
        Uefi => "uefi",
        Unknown => "unknown",
        /// Apple spatial OS
        VisionOS => "visionos",
        /// PlayStation Vita OS
        Vita => "vita",
        /// VVxWorks is a deterministic, priority-based preemptive RTOS with low latency and minimal jitter
        VxWorks => "vxworks",
        /// The WebAssembly System Interface
        Wasi => "wasi",
        /// Apple Watch OS
        WatchOS => "watchos",
        /// Microsoft’s Windows operating system
        Windows => "windows",
        /// Privacy-focused microkernel OS
        Xous => "xous",
        /// Zero-knowledge proof VM
        Zkvm => "zkvm",
    }

    as_str_doc = "String representing this target OS which matches `#[cfg(target_os)]`",
    from_str_doc = "Tries to parse the given string as an [`Os`] falling back to [`Os::Other`] for unknown values",
}

impl Os {
    /// Gets the current target [`Os`].
    #[must_use]
    pub fn target() -> Self {
        Self::from_str(build_env("CARGO_CFG_TARGET_OS"))
    }
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
