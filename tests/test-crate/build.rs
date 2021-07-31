use build_target::{Arch, Env, Family, Os};

fn main() {
    assert_eq!(format!("{:#?}", Arch::target().unwrap()), include_str!("../test_data/arch.txt"));
    assert_eq!(format!("{:#?}", Env::target().unwrap()), include_str!("../test_data/env.txt"));
    assert_eq!(format!("{:#?}", Os::target().unwrap()), include_str!("../test_data/os.txt"));
    assert_eq!(format!("{:#?}", Family::target().unwrap()), include_str!("../test_data/family.txt"));
}
