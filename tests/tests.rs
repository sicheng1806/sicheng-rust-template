use std::fs;
use std::path::Path;
use std::process::Command;

fn prepare_tempdir() {
    let temp_dir = Path::new(".tmp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).expect("Failed to remove exisiting temp directory");
        fs::create_dir(&temp_dir).expect("Failed to create temp directory");
    } else {
        fs::create_dir(temp_dir).expect("Failed to create temp directory");
    }
}

#[test]
fn generate_template() {
    prepare_tempdir();
    let temp_dir = Path::new(".tmp");
    let status = Command::new("cargo")
        .args(["generate", "-p", "../template", "-n", "test-demo"])
        .current_dir(temp_dir)
        .status()
        .expect("Failed to execute `cargo generate`");
    println!("hello world {}", status.success());
    assert!(status.success());
}
