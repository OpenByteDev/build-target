use std::{path::PathBuf, process::Command, str::FromStr};

#[test]
fn test_build_all_installed_targets() {
    let crate_path = PathBuf::from_str("tests")
        .unwrap()
        .join("test-crate")
        .canonicalize()
        .unwrap();

    let installed_targets = get_installed_targets();

    for target in installed_targets {
        println!("Building for target: {target}");

        let status = Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg(&target)
            .current_dir(&crate_path)
            .status()
            .expect("Failed to run cargo");

        assert!(status.success(), "❌ Build failed for target: {target}");
    }
}

fn get_installed_targets() -> Vec<String> {
    let output = Command::new("rustup")
        .args(&["target", "list", "--installed"])
        .output()
        .expect("Failed to list installed targets");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}
