use crate::rs;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

pub fn run_tpch(query: u8, sf: &str) -> io::Result<()> {
    let path = format!("{}/progs/tpch/q{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path)?.replace(
        "datasets/tpch/",
        &format!("../../datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    run(query, &code)
}

pub fn run(query: u8, code: &str) -> io::Result<()> {
    let name = &format!("q{query}");
    let path = format!("generated/{name}/{name}.rs");
    if let Some(parent) = std::path::Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?.write_all(code.as_bytes())?;
    cargo_toml(&name)?;
    cargo_run(name)
}

fn cargo_run(name: &str) -> io::Result<()> {
    let dir_path = &format!("generated/{name}");
    let output = Command::new("cargo")
        .current_dir(dir_path)
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg(name)
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        io::stdout().write_all(stdout.as_bytes())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.replace(&format!("{name}.rs:"), &format!("{dir_path}/{name}.rs:"));
        io::stderr().write_all(stderr.as_bytes())?;
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}

fn cargo_toml(name: &str) -> io::Result<()> {
    let contents = toml_contents(name);
    let path = format!("generated/{name}/cargo.toml");
    File::create(path)?.write_all(contents.as_bytes())
}

fn toml_contents(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
sdql_runtime = {{ path = "../../runtime" }}

[[bin]]
name = "{name}"
path = "{name}.rs"
"#
    )
}
