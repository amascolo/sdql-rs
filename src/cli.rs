use crate::rs;
use std::fs::{self, File};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn run_tpch(query: u8, sf: &str) -> io::Result<String> {
    let path = format!("{}/progs/tpch/{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path)?.replace(
        "datasets/tpch/",
        &format!("datasets/tpch_datasets/SF_{sf}/"),
    );
    let code = rs!(&src);
    let name = filename(&code);
    run(&name, &code)
}

pub fn filename(code: &str) -> String {
    let hash = {
        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
    };
    format!("{hash:x}")
}

pub fn run(name: &str, code: &str) -> io::Result<String> {
    write_if_different(name, code)?; // avoids triggering recompilation
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--bin")
        .arg(name)
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
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
