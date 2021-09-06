mod arch;
pub use arch::*;

mod env;
pub use env::*;

mod os;
pub use os::*;

mod family;
pub use family::*;

mod target;
pub use target::*;

mod utils;

/// Gets the current target [`Arch`]. This function is equivalent to [`Arch::target()`].
pub fn target_arch() -> Result<Arch<'static>, std::env::VarError> {
    Arch::target()
} 
/// Gets the current target [`Env`]. This function is equivalent to [`Env::target()`].
pub fn target_env() -> Result<Env<'static>, std::env::VarError> {
    Env::target()
}
/// Gets the current target [`Os`]. This function is equivalent to [`Os::target()`].
pub fn target_os() -> Result<Os<'static>, std::env::VarError> {
    Os::target()
}
/// Gets the current target [`Family`]. This function is equivalent to [`Family::target()`].
pub fn target_family() -> Result<Family<'static>, std::env::VarError> {
    Family::target()
}
/// Gets the current target triple.
pub fn target_triple() -> Result<String, std::env::VarError> {
    std::env::var("TARGET")
} 
/// Gets the current target information as a [`Target`]. This function is equivalent to [`Target::current()`].
pub fn target() -> Result<Target<'static>, std::env::VarError> {
    Target::current()
}
