use crate::rs;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn run_tpch(query: u8, sf: &str) -> () {
    let path = format!("{}/progs/tpch/q{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path).unwrap().replace(
        "datasets/tpch/",
        &format!("../datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    run(&code);
}

pub fn run(code: &str) -> () {
    File::create("generated/src/main.rs")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .current_dir("generated")
        .output()
        .unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{stdout}");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.replace("--> src/main.rs:", "--> generated/src/main.rs:");
        eprintln!("{stderr}");
    }
}
