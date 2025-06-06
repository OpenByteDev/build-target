#![allow(clippy::needless_doctest_main, clippy::should_implement_trait)]

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
//!         build_target::target().unwrap()
//!     );
//! }
//! ```
//!
//! Gets the parts of the current build target individually.
//! ```rust no_run
//! // inside build.rs
//!
//! fn main() {
//!     let arch   = build_target::target_arch().unwrap();   // eg. "x86_64", "aarch64", ...
//!     let endian = build_target::target_endian().unwrap(); // eg. "big", "little", ...
//!     let env    = build_target::target_env().unwrap();    // eg. "gnu", "msvc", ...
//!     let family = build_target::target_family().unwrap(); // eg. "windows", "unix", ...
//!     let pw     = build_target::target_pointer_width().unwrap(); // eg. "32", "64", ...
//!     let os     = build_target::target_os().unwrap();     // eg. "android", "linux", ...
//!     let vendor = build_target::target_vendor().unwrap(); // eg. "apple", "unknown", ...
//!     let triple = build_target::target_triple().unwrap(); // eg. "x86_64-unknown-linux-gnu", ...
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

mod utils;

/// Gets the current target [`Arch`]. This function is equivalent to [`Arch::target()`].
pub fn target_arch() -> Result<Arch<'static>, std::env::VarError> {
    Arch::target()
}
/// Gets the current target [`Endian`]. This function is equivalent to [`Endian::target()`].
pub fn target_endian() -> Result<Endian<'static>, std::env::VarError> {
    Endian::target()
}
/// Gets the current target [`Env`]. This function is equivalent to [`Env::target()`].
pub fn target_env() -> Result<Env<'static>, std::env::VarError> {
    Env::target()
}
/// Gets the current target [`Family`]. This function is equivalent to [`Family::target()`].
pub fn target_family() -> Result<Family<'static>, std::env::VarError> {
    Family::target()
}
/// Gets the current target [`Os`]. This function is equivalent to [`Os::target()`].
pub fn target_os() -> Result<Os<'static>, std::env::VarError> {
    Os::target()
}
/// Gets the current target [`PointerWidth`]. This function is equivalent to [`PointerWidth::target()`].
pub fn target_pointer_width() -> Result<PointerWidth<'static>, std::env::VarError> {
    PointerWidth::target()
}
/// Gets the current target [`Vendor`]. This function is equivalent to [`Vendor::target()`].
pub fn target_vendor() -> Result<Vendor<'static>, std::env::VarError> {
    Vendor::target()
}
/// Gets the current target triple.
pub fn target_triple() -> Result<String, std::env::VarError> {
    std::env::var("TARGET")
}
/// Gets the current target information as a [`Target`]. This function is equivalent to [`Target::current()`].
pub fn target() -> Result<Target<'static>, std::env::VarError> {
    Target::current()
}
