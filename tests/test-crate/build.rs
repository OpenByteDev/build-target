use build_target::{target_triple, Arch, Endian, Env, Family, Os, PointerWidth, Vendor};
use velcro::hash_map;

fn main() {
    let map = hash_map! {
        // Windows targets
        "i686-pc-windows-msvc":
            (Arch::X86, Endian::Little, Env::Msvc, Family::Windows, Os::Windows, PointerWidth::U32, Vendor::Pc),
        "i686-pc-windows-gnu":
            (Arch::X86, Endian::Little, Env::Gnu, Family::Windows, Os::Windows, PointerWidth::U32, Vendor::Pc),
        "x86_64-pc-windows-msvc":
            (Arch::X86_64, Endian::Little, Env::Msvc, Family::Windows, Os::Windows, PointerWidth::U64, Vendor::Pc),
        "x86_64-pc-windows-gnu":
            (Arch::X86_64, Endian::Little, Env::Gnu, Family::Windows, Os::Windows, PointerWidth::U64, Vendor::Pc),

        // Linux targets
        "x86_64-unknown-linux-gnu":
            (Arch::X86_64, Endian::Little, Env::Gnu, Family::Unix, Os::Linux, PointerWidth::U64, Vendor::Unknown),
        "x86_64-unknown-linux-musl":
            (Arch::X86_64, Endian::Little, Env::Musl, Family::Unix, Os::Linux, PointerWidth::U64, Vendor::Unknown),
        "aarch64-unknown-linux-gnu":
            (Arch::AArch64, Endian::Little, Env::Gnu, Family::Unix, Os::Linux, PointerWidth::U64, Vendor::Unknown),
        "armv7-unknown-linux-gnueabihf":
            (Arch::Arm, Endian::Little, Env::Gnu, Family::Unix, Os::Linux, PointerWidth::U32, Vendor::Unknown),

        // macOS targets
        "x86_64-apple-darwin":
            (Arch::X86_64, Endian::Little, Env::Unknown, Family::Unix, Os::MacOS, PointerWidth::U64, Vendor::Apple),
        "aarch64-apple-darwin":
            (Arch::AArch64, Endian::Little, Env::Unknown, Family::Unix, Os::MacOS, PointerWidth::U64, Vendor::Apple),

        // WASM targets
        "wasm32-unknown-unknown":
            (Arch::Wasm32, Endian::Little, Env::Unknown, Family::Unknown, Os::Unknown, PointerWidth::U32, Vendor::Unknown),
        "wasm32-wasi":
            (Arch::Wasm32, Endian::Little, Env::Unknown, Family::Unknown, Os::Wasi, PointerWidth::U32, Vendor::Unknown),
    };

    let target = target_triple().unwrap();
    let data_for_target = map.get(target.as_str()).expect("Target not found in map");

    assert_eq!(
        *data_for_target,
        (
            Arch::target().unwrap(),
            Endian::target().unwrap(),
            Env::target().unwrap(),
            Family::target().unwrap(),
            Os::target().unwrap(),
            PointerWidth::target().unwrap(),
            Vendor::target().unwrap()
        )
    );
}
