use std::{borrow::Cow, env::VarError};

use crate::{Arch, Endian, Env, Family, Os, PointerWidth, Vendor, target_triple};

/// Combined information about a build target.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Target<'a> {
    /// The architecture of the target, such as `x86_64`, `aarch64`, or `i686`.
    pub arch: Arch<'a>,
    /// The endianness of the target architecture, such as `little` or `big`.
    pub endian: Endian<'a>,
    /// The environment of the target, such as `gnu`, `msvc`, or `none`.
    pub env: Env<'a>,
    /// The operating system of the target, such as `linux`, `windows`, or `macos`.
    pub os: Os<'a>,
    /// The pointer width of the target, such as `32` or `64`.
    pub pointer_width: PointerWidth<'a>,
    /// The family of the target, such as `unix`, `windows`, or `wasm`.
    pub family: Family<'a>,
    /// The vendor of the target, such as `apple`, `unknown`, or `pc`.
    pub vendor: Vendor<'a>,
    /// The target triple, which is a string that uniquely identifies the target.
    pub triple: Cow<'a, str>,
}

impl<'a> Target<'a> {
    /// Gets the current build target as a [`Target`].
    pub fn current() -> Result<Self, VarError> {
        Ok(Self {
            arch: Arch::target()?,
            endian: Endian::target()?,
            env: Env::target()?,
            os: Os::target()?,
            pointer_width: PointerWidth::target()?,
            family: Family::target()?,
            vendor: Vendor::target()?,
            triple: target_triple()?.into(),
        })
    }
}
