use std::{error::Error, path::PathBuf, process::Command, str::FromStr};

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let crate_path = PathBuf::from_str("tests")?.join("test-crate").canonicalize()?;

    Command::new("cargo")
        .arg("build")
        .current_dir(&crate_path)
        .spawn()?
        .wait()?;

    Ok(())
}
