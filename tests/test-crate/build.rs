use build_target::{target_triple, Target, Arch, Endian, Env, Family, Os, PointerWidth, Vendor};
use velcro::hash_map;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let map = hash_map! {
        // Windows targets
        "i686-pc-windows-msvc": Target {
            arch: Arch::X86,
            endian: Endian::Little,
            env: Some(Env::Msvc),
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Pc,
            triple: "i686-pc-windows-msvc".to_string(),
        },
        "i686-pc-windows-gnu": Target {
            arch: Arch::X86,
            endian: Endian::Little,
            env: Some(Env::Gnu),
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Pc,
            triple: "i686-pc-windows-gnu".to_string(),
        },
        "x86_64-pc-windows-msvc": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Some(Env::Msvc),
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Pc,
            triple: "x86_64-pc-windows-msvc".to_string(),
        },
        "x86_64-pc-windows-gnu": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Some(Env::Gnu),
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Pc,
            triple: "x86_64-pc-windows-gnu".to_string(),
        },

        // Linux targets
        "x86_64-unknown-linux-gnu": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Some(Env::Gnu),
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Unknown,
            triple: "x86_64-unknown-linux-gnu".to_string(),
        },
        "x86_64-unknown-linux-musl": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Some(Env::Musl),
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Unknown,
            triple: "x86_64-unknown-linux-musl".to_string(),
        },
        "aarch64-unknown-linux-gnu": Target {
            arch: Arch::AArch64,
            endian: Endian::Little,
            env: Some(Env::Gnu),
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Unknown,
            triple: "aarch64-unknown-linux-gnu".to_string(),
        },
        "armv7-unknown-linux-gnueabihf": Target {
            arch: Arch::Arm,
            endian: Endian::Little,
            env: Some(Env::Gnu),
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: "armv7-unknown-linux-gnueabihf".to_string(),
        },

        // macOS targets
        "x86_64-apple-darwin": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: None,
            family: vec![Family::Unix],
            os: Os::MacOS,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Apple,
            triple: "x86_64-apple-darwin".to_string(),
        },
        "aarch64-apple-darwin": Target {
            arch: Arch::AArch64,
            endian: Endian::Little,
            env: None,
            family: vec![Family::Unix],
            os: Os::MacOS,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Apple,
            triple: "aarch64-apple-darwin".to_string(),
        },

        // Others
        "riscv32imac-unknown-none-elf": Target {
            arch: Arch::Riscv32,
            endian: Endian::Little,
            env: None,
            family: vec![],
            os: Os::None,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: "riscv32imac-unknown-none-elf".to_string(),
        },
        "thumbv7m-none-eabi": Target {
            arch: Arch::Arm,
            endian: Endian::Little,
            env: None,
            family: vec![],
            os: Os::None,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: "thumbv7m-none-eabi".to_string(),
        },
        "wasm32-unknown-emscripten": Target {
            arch: Arch::Wasm32,
            endian: Endian::Little,
            env: None,
            family: vec![Family::Unix, Family::Wasm],
            os: Os::Emscripten,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: "wasm32-unknown-emscripten".to_string(),
        },
    };

    let target_str = target_triple();
    let target = Target::current();

    p!("{}", target_str);
    p!("{:?}", target);

    if let Some(expected_target) = map.get(target_str.as_str()) {
        assert_eq!(
            &target,
            expected_target,
            "❌ Target data does not match expected metadata!"
        );
        p!("✅ Target metadata matched expected values.");
    } else {
        p!("⚠️  No known expected Target for {}", target_str);
    }
}
