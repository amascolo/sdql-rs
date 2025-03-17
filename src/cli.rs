use crate::rs;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn run_tpch(query: u8, sf: &str) -> io::Result<()> {
    let path = format!("{}/progs/tpch/q{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path)?.replace(
        "datasets/tpch/",
        &format!("../datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    run(query, &code)
}

pub fn run(query: u8, code: &str) -> io::Result<()> {
    let name = &format!("q{query}");
    let path = &format!("generated/src/bin/{name}.rs");
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?.write_all(code.as_bytes())?;
    cargo_run(name)
}

fn cargo_run(name: &str) -> io::Result<()> {
    let output = cargo_cmd().arg(name).output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        io::stdout().write_all(stdout.as_bytes())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.replace(&format!("{name}.rs:"), &format!("generated/{name}.rs:"));
        io::stderr().write_all(stderr.as_bytes())?;
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}

fn cargo_cmd() -> Command {
    let mut cmd = Command::new("cargo");
    cmd.current_dir("generated")
        .arg("run")
        .arg("--release")
        .arg("--bin");
    cmd
}
