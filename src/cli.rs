use crate::rs;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn run_tpch(query: u8, sf: &str) -> io::Result<()> {
    let path = format!("{}/progs/tpch/{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path)?.replace(
        "datasets/tpch/",
        &format!("datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    run(&query.to_string(), &code)
}

pub fn run(query: &str, code: &str) -> io::Result<()> {
    write_if_different(query, code)?;
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(query)
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        io::stdout().write_all(stdout.as_bytes())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        io::stderr().write_all(stderr.as_bytes())?;
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}

fn write_if_different(query: &str, code: &str) -> io::Result<()> {
    let path = format!("src/bin/{query}.rs");
    let path = Path::new(&path);
    if path.exists() && fs::read(path)? == code.as_bytes() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?.write_all(code.as_bytes())
}
