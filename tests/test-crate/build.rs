use build_target::{target_triple, Target, Arch, Endian, Env, Family, Os, PointerWidth, Vendor};
use velcro::hash_map;
use std::borrow::Cow;

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
            env: Env::Msvc,
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Pc,
            triple: Cow::from("i686-pc-windows-msvc"),
        },
        "i686-pc-windows-gnu": Target {
            arch: Arch::X86,
            endian: Endian::Little,
            env: Env::Gnu,
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Pc,
            triple: Cow::from("i686-pc-windows-gnu"),
        },
        "x86_64-pc-windows-msvc": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Env::Msvc,
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Pc,
            triple: Cow::from("x86_64-pc-windows-msvc"),
        },
        "x86_64-pc-windows-gnu": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Env::Gnu,
            family: vec![Family::Windows],
            os: Os::Windows,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Pc,
            triple: Cow::from("x86_64-pc-windows-gnu"),
        },

        // Linux targets
        "x86_64-unknown-linux-gnu": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Env::Gnu,
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Unknown,
            triple: Cow::from("x86_64-unknown-linux-gnu"),
        },
        "x86_64-unknown-linux-musl": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Env::Musl,
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Unknown,
            triple: Cow::from("x86_64-unknown-linux-musl"),
        },
        "aarch64-unknown-linux-gnu": Target {
            arch: Arch::AArch64,
            endian: Endian::Little,
            env: Env::Gnu,
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Unknown,
            triple: Cow::from("aarch64-unknown-linux-gnu"),
        },
        "armv7-unknown-linux-gnueabihf": Target {
            arch: Arch::Arm,
            endian: Endian::Little,
            env: Env::Gnu,
            family: vec![Family::Unix],
            os: Os::Linux,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: Cow::from("armv7-unknown-linux-gnueabihf"),
        },

        // macOS targets
        "x86_64-apple-darwin": Target {
            arch: Arch::X86_64,
            endian: Endian::Little,
            env: Env::from_str(""),
            family: vec![Family::Unix],
            os: Os::MacOS,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Apple,
            triple: Cow::from("x86_64-apple-darwin"),
        },
        "aarch64-apple-darwin": Target {
            arch: Arch::AArch64,
            endian: Endian::Little,
            env: Env::from_str(""),
            family: vec![Family::Unix],
            os: Os::MacOS,
            pointer_width: PointerWidth::U64,
            vendor: Vendor::Apple,
            triple: Cow::from("aarch64-apple-darwin"),
        },

        // Others
        "riscv32imac-unknown-none-elf": Target {
            arch: Arch::Riscv32,
            endian: Endian::Little,
            env: Env::from_str(""),
            family: vec![],
            os: Os::None,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: Cow::from("riscv32imac-unknown-none-elf"),
        },
        "thumbv7m-none-eabi": Target {
            arch: Arch::Arm,
            endian: Endian::Little,
            env: Env::from_str(""),
            family: vec![],
            os: Os::None,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: Cow::from("thumbv7m-none-eabi"),
        },
        "wasm32-unknown-emscripten": Target {
            arch: Arch::Wasm32,
            endian: Endian::Little,
            env: Env::from_str(""),
            family: vec![Family::Unix, Family::Wasm],
            os: Os::Emscripten,
            pointer_width: PointerWidth::U32,
            vendor: Vendor::Unknown,
            triple: Cow::from("wasm32-unknown-emscripten"),
        },
    };

    let target_str = target_triple().unwrap();
    let target = Target::current().unwrap();

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
