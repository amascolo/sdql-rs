use std::process::Command;

fn main() {
    assert!(
        Command::new("cargo")
            .current_dir("generated")
            .arg("build")
            .status()
            .unwrap()
            .success()
    );
}
