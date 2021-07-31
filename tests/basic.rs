use std::{error::Error, fmt, fs, path::PathBuf, process::Command, str::FromStr};

use build_target::{Arch, Env, Family, Os};
use serial_test::serial;

#[test]
#[serial]
fn i686_pc_windows_msvc() -> Result<(), Box<dyn Error>> {
    test_target("i686-pc-windows-msvc", Arch::X86, Env::MSVC, Family::Windows, Os::Windows)
}

#[test]
#[serial]
fn x86_64_pc_windows_gnu() -> Result<(), Box<dyn Error>> {
    test_target("x86_64-pc-windows-gnu", Arch::X86_64, Env::GNU, Family::Windows, Os::Windows)
}

#[test]
#[serial]
fn x86_64_unknown_linux_musl() -> Result<(), Box<dyn Error>> {
    test_target("x86_64-unknown-linux-musl", Arch::X86_64, Env::Musl, Family::Unix, Os::Linux)
}


static TEST_CRATE_NAME: &str = "test-crate";

fn test_target(target: &str, arch: Arch, env: Env, family: Family, os: Os) -> Result<(), Box<dyn Error>> {
    let crate_path = PathBuf::from_str("./tests")?.join(TEST_CRATE_NAME).canonicalize()?;

    format_struct_into_test_data_file(arch, "arch");
    format_struct_into_test_data_file(env, "env");
    format_struct_into_test_data_file(family, "family");
    format_struct_into_test_data_file(os, "os");

    Command::new("cargo")
        .arg("test")
        .arg("--target").arg(target)
        .current_dir(&crate_path)
        .spawn()?
        .wait()?;

    Ok(())
}

fn format_struct_into_test_data_file(obj: impl fmt::Debug, file_name: &str) {
    fs::write(format!("tests/test_data/{}.txt", file_name), format!("{:#?}", obj)).unwrap()
}
