use std::{path::PathBuf, process::Command, str::FromStr};

#[test]
fn test() {
    let crate_path = PathBuf::from_str("tests")
        .unwrap()
        .join("test-crate")
        .canonicalize()
        .unwrap();

    let exist_status = Command::new("cargo")
        .arg("build")
        .current_dir(&crate_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    assert!(exist_status.success(), "Failed to build test crate");
}
