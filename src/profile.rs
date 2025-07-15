use std::fmt;

use crate::utils::{build_env, define_target_enum};

define_target_enum! {
    /// Profile of the current build.
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Profile {
        /// The dev profile is used for normal development and debugging.
        /// It is the default for build commands like `cargo build`.
        Dev => "dev",

        /// The release profile is intended for optimized artifacts used for releases and in production.
        /// This profile is used when the `--release` flag is used, and is the default for `cargo install`.
        Release => "release",

        /// The test profile is used for building tests, or when benchmarks are built in debug mode with `cargo build`.
        Test => "test",

        /// The bench profile is used for building benchmarks, or when tests are built with the `--release` flag.
        Bench => "bench",
    }

    as_str_doc = "String representing this target profile.",
    from_str_doc = "Tries to parse the given string as an [`Profile`] falling back to [`Profile::Other`] for unknown values.",
}

impl Profile {
    /// Gets the current [`Profile`].
    #[must_use]
    pub fn current() -> Self {
        Self::from_str(build_env("PROFILE"))
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
