use crate::rs;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn run_tpch(name: u8, sf: &str) -> io::Result<()> {
    let path = format!("{}/progs/tpch/{name}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path)?.replace(
        "datasets/tpch/",
        &format!("datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    run(&name.to_string(), &code)
}

pub fn run(name: &str, code: &str) -> io::Result<()> {
    write_if_different(name, code)?; // avoids triggering recompilation
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--bin")
        .arg(name)
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

fn write_if_different(name: &str, code: &str) -> io::Result<()> {
    let path = format!("src/bin/{name}.rs");
    let path = Path::new(&path);
    if path.exists() && fs::read(path)? == code.as_bytes() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?.write_all(code.as_bytes())
}
