use crate::{Arch, Endian, Env, Family, Os, PointerWidth, Vendor, target_triple};

/// Combined information about a build target.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Target {
    /// The architecture of the target, such as `x86_64`, `aarch64`, or `i686`.
    pub arch: Arch,
    /// The endianness of the target architecture, such as `little` or `big`.
    pub endian: Endian,
    /// The environment of the target, such as `gnu`, `msvc`, or `none`.
    pub env: Option<Env>,
    /// The operating system of the target, such as `linux`, `windows`, or `macos`.
    pub os: Os,
    /// The pointer width of the target, such as `32` or `64`.
    pub pointer_width: PointerWidth,
    /// The family of the target, such as `unix`, `windows`, or `wasm`.
    pub family: Vec<Family>,
    /// The vendor of the target, such as `apple`, `unknown`, or `pc`.
    pub vendor: Vendor,
    /// The target triple, which is a string that uniquely identifies the target.
    pub triple: String,
}

impl Target {
    /// Gets the current build target as a [`Target`].
    #[must_use]
    pub fn current() -> Self {
        Self {
            arch: Arch::target(),
            endian: Endian::target(),
            env: Env::target(),
            os: Os::target(),
            pointer_width: PointerWidth::target(),
            family: Family::target(),
            vendor: Vendor::target(),
            triple: target_triple(),
        }
    }
}
