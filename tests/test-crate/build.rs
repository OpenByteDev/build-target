use build_target::{target_triple, Arch, Endian, Env, Family, Os, PointerWidth};
use velcro::hash_map;

fn main() {
    let map = hash_map! {
        "i686-pc-windows-msvc": (Arch::X86, Endian::Little, Env::Msvc, Family::Windows, Os::Windows, PointerWidth::U32),
        "i686-pc-windows-gnu": (Arch::X86, Endian::Little, Env::Gnu, Family::Windows, Os::Windows, PointerWidth::U32),
        "x86_64-pc-windows-msvc": (Arch::X86_64, Endian::Little, Env::Msvc, Family::Windows, Os::Windows, PointerWidth::U64),
        "x86_64-pc-windows-gnu": (Arch::X86_64, Endian::Little, Env::Gnu, Family::Windows, Os::Windows, PointerWidth::U64),
        "x86_64-unknown-linux-gnu": (Arch::X86_64, Endian::Little, Env::Gnu, Family::Unix, Os::Linux, PointerWidth::U64),
        "x86_64-unknown-linux-musl": (Arch::X86_64, Endian::Little, Env::Musl, Family::Unix, Os::Linux, PointerWidth::U64),
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
            PointerWidth::target().unwrap()
        )
    );
}
