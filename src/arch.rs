use crate::utils::define_target_enum;
use std::{
    env::{self, VarError},
    fmt,
};

define_target_enum! {
    /// Target CPU architecture
    #[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Arch<'a> {
        /// ARMv8 64-bit architecture
        AArch64 => "aarch64",
        /// AMD GPU architecture
        Amdgpu => "amdgpu",
        /// 32-bit ARM architecture
        Arm => "arm",
        /// ARM64 architecture with Windows EC (Emulation Compatible)
        Arm64ec => "arm64ec",
        /// AVR 8-bit microcontroller architecture
        Avr => "avr",
        /// Berkeley Packet Filter virtual machine architecture
        Bpf => "bpf",
        /// C-SKY CPU architecture
        Csky => "csky",
        /// Qualcomm Hexagon DSP architecture
        Hexagon => "hexagon",
        /// LoongArch 64-bit CPU architecture
        Loongarch64 => "loongarch64",
        /// Motorola 68k CPU architecture
        M68k => "m68k",
        /// 32-bit MIPS CPU architecture
        Mips => "mips",
        /// MIPS 32-bit Revision 6 architecture
        Mips32r6 => "mips32r6",
        /// 64-bit MIPS CPU architecture
        Mips64 => "mips64",
        /// MIPS 64-bit Revision 6 architecture
        Mips64r6 => "mips64r6",
        /// 16-bit MSP430 microcontroller architecture
        Msp430 => "msp430",
        /// 64-bit NVIDIA PTX virtual architecture
        Nvptx64 => "nvptx64",
        /// 32-bit POWERPC architecture
        PowerPc => "powerpc",
        /// 64-bit POWERPC architecture
        PowerPc64 => "powerpc64",
        /// 32-bit RISC-V architecture
        Riscv32 => "riscv32",
        /// 64-bit RISC-V architecture
        Riscv64 => "riscv64",
        /// 64-bit IBM z/Architecture mainframe CPU
        S390X => "s390x",
        /// 32-bit SPARC CPU architecture
        Sparc => "sparc",
        /// 64-bit SPARC CPU architecture
        Sparc64 => "sparc64",
        /// 32-bit WebAssembly architecture
        Wasm32 => "wasm32",
        /// 64-bit WebAssembly architecture
        Wasm64 => "wasm64",
        /// Generic 32-bit x86 CPU architecture
        X86 => "x86",
        /// 64-bit x86-64 (AMD64) CPU architecture
        X86_64 => "x86_64",
        /// Xtensa CPU architecture (commonly used in embedded systems)
        Xtensa => "xtensa",
    }

    as_str_doc = "String representing this target architecture which matches `#[cfg(target_arch)]`.",
    from_str_doc = "Tries to parse the given string as an [`Arch`] falling back to [`Arch::Other`] for unknown values.",
}

impl Arch<'_> {
    /// Gets the current target [`Arch`].
    pub fn target() -> Result<Self, VarError> {
        env::var("CARGO_CFG_TARGET_ARCH").map(Self::from_str)
    }
}

impl fmt::Display for Arch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
