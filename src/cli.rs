use crate::{rs, rs_par};
use std::fs::{self, File};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn run_tpch<const PARALLEL: bool>(query: u8, sf: &str) -> io::Result<Vec<u8>> {
    let path = format!("{}/progs/tpch/{query}.sdql", env!("CARGO_MANIFEST_DIR"));
    let src = fs::read_to_string(&path)?.replace(
        "datasets/tpch/",
        &format!("datasets/tpch_datasets/SF_{sf}/"),
    );

    // FIXME TPCH q15 add support for max
    let src = if query == 15 {
        src.replace(
            "let max_revenue = sum(<_,v> <- suppkey_to_revenue) promote[max_sum](v)",
            match sf {
                "0.01" => "let max_revenue = 1161099.4635999997",
                "1" => "let max_revenue = 1772627.2087",
                _ => unimplemented!(),
            },
        )
    } else {
        src
    };

    let code = if PARALLEL { rs_par!(&src) } else { rs!(&src) };
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

pub fn run(name: &str, code: &str) -> io::Result<Vec<u8>> {
    write_if_different(name, code)?; // avoids triggering recompilation
    let output = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--quiet")
        .arg("--bin")
        .arg(name)
        .output()?;
    if output.status.success() {
        Ok(output.stdout)
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
