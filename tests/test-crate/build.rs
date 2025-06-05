use build_target::{target_triple, Arch, Env, Family, Os};
use velcro::hash_map;

fn main() {
    let map = hash_map! {
        "i686-pc-windows-msvc": (Arch::X86, Env::Msvc, Family::Windows, Os::Windows),
        "i686-pc-windows-gnu": (Arch::X86, Env::Gnu, Family::Windows, Os::Windows),
        "x86_64-pc-windows-msvc": (Arch::X86_64, Env::Msvc, Family::Windows, Os::Windows),
        "x86_64-pc-windows-gnu": (Arch::X86_64, Env::Gnu, Family::Windows, Os::Windows),
        "x86_64-unknown-linux-gnu": (Arch::X86_64, Env::Gnu, Family::Unix, Os::Linux),
        "x86_64-unknown-linux-musl": (Arch::X86_64, Env::Musl, Family::Unix, Os::Linux),
    };

    let target = target_triple().unwrap();
    let data_for_target = map.get(target.as_str());
    if data_for_target.is_none() {
        return;
    }
    let data_for_target = data_for_target.unwrap();

    assert_eq!(
        *data_for_target,
        (Arch::target().unwrap(), Env::target().unwrap(), Family::target().unwrap(), Os::target().unwrap())
    );
}
