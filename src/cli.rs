use crate::rs;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

pub fn run_tpch(query: u8, sf: &str) -> Option<()> {
    let path = format!("{}/progs/tpch/q{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path).unwrap().replace(
        "datasets/tpch/",
        &format!("../datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    run(query, &code)
}

pub fn run(query: u8, code: &str) -> Option<()> {
    let name = &format!("q{query}");

    let path = format!("generated/{name}.rs");
    File::create(path).ok()?.write_all(code.as_bytes()).ok()?;

    create_cargo_toml(&name)?;
    cargo_run(name)
}

fn cargo_run(name: &str) -> Option<()> {
    let output = cargo_cmd().arg(name).output().ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        io::stdout().write_all(stdout.as_bytes()).ok()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.replace(&format!("{name}.rs:"), &format!("generated/{name}.rs:"));
        io::stderr().write_all(stderr.as_bytes()).ok()?;
        None
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

fn create_cargo_toml(name: &str) -> Option<()> {
    let cargo = CARGO_TOML.to_owned() + &bin_toml(&name);
    let cargo = cargo.as_bytes();
    File::create(CARGO_PATH).ok()?.write_all(cargo).ok()
}

const CARGO_PATH: &str = "generated/cargo.toml";

const CARGO_TOML: &str = r#"[package]
name = "generated"
version = "0.1.0"
edition = "2024"

[dependencies]
sdql_runtime = { path = "../runtime" }
"#;

fn bin_toml(name: &str) -> String {
    format!(
        r#"
[[bin]]
name = "{name}"
path = "{name}.rs"
"#
    )
}
