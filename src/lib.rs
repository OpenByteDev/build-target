#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_doctest_main,
    clippy::missing_errors_doc,
    clippy::should_implement_trait,
    clippy::doc_markdown
)]

//! A crate that provides programmatic access to information about the current build target inside `build.rs`.
//!
//! ## Examples
//! Prints all available information about the current build target.
//! ```rust no_run
//! // inside build.rs
//!
//! fn main() {
//!     // The panic is just used to print the information to the console.
//!     panic!("current build target: {:#?}",
//!         build_target::target()
//!     );
//! }
//! ```
//!
//! Gets the parts of the current build target individually.
//! ```rust no_run
//! // inside build.rs
//!
//! fn main() {
//!     let arch   = build_target::target_arch();   // eg. "x86_64", "aarch64", ...
//!     let endian = build_target::target_endian(); // eg. "big", "little", ...
//!     let env    = build_target::target_env();    // eg. "gnu", "msvc", ...
//!     let family = build_target::target_family(); // eg. "windows", "unix", ...
//!     let pw     = build_target::target_pointer_width(); // eg. "32", "64", ...
//!     let os     = build_target::target_os();     // eg. "android", "linux", ...
//!     let vendor = build_target::target_vendor(); // eg. "apple", "unknown", ...
//!     let triple = build_target::target_triple(); // eg. "x86_64-unknown-linux-gnu", ...
//! }
//! ```

mod arch;
pub use arch::*;

mod endian;
pub use endian::*;

mod env;
pub use env::*;

mod family;
pub use family::*;

mod os;
pub use os::*;

mod pointer_width;
pub use pointer_width::*;

mod profile;
pub use profile::*;

mod vendor;
pub use vendor::*;

mod target;
pub use target::*;

use crate::utils::build_env;

mod utils;

/// Gets the current target [`Arch`]. This function is equivalent to [`Arch::target()`].
#[must_use]
pub fn target_arch() -> Arch {
    Arch::target()
}
/// Gets the current target [`Endian`]. This function is equivalent to [`Endian::target()`].
#[must_use]
pub fn target_endian() -> Endian {
    Endian::target()
}
/// Gets the current target [`Env`]. This function is equivalent to [`Env::target()`].
#[must_use]
pub fn target_env() -> Option<Env> {
    Env::target()
}
/// Gets the current target [`Family`]s. This function is equivalent to [`Family::target()`].
#[must_use]
pub fn target_family() -> Vec<Family> {
    Family::target()
}
/// Gets the current target [`Os`]. This function is equivalent to [`Os::target()`].
#[must_use]
pub fn target_os() -> Os {
    Os::target()
}
/// Gets the current target [`PointerWidth`]. This function is equivalent to [`PointerWidth::target()`].
#[must_use]
pub fn target_pointer_width() -> PointerWidth {
    PointerWidth::target()
}
/// Gets the current target [`Vendor`]. This function is equivalent to [`Vendor::target()`].
#[must_use]
pub fn target_vendor() -> Vendor {
    Vendor::target()
}
/// Gets the current target triple.
#[must_use]
pub fn target_triple() -> String {
    build_env("TARGET")
}
/// Gets the current target information as a [`Target`]. This function is equivalent to [`Target::current()`].
#[must_use]
pub fn target() -> Target {
    Target::current()
}
