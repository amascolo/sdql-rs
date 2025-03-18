use clap::Parser;
use sdql::cli::run;
use sdql::rs;
use std::hash::{DefaultHasher, Hash, Hasher};

use std::{fs, io};
#[derive(Parser)]
struct Args {
    sdql_path: String,
}

fn main() -> io::Result<()> {
    let path = Args::parse().sdql_path;
    let src = fs::read_to_string(&path)?;
    let name = filename(&src);
    let code = rs!(&src);
    run(&name, &code)
}

fn filename(src: &str) -> String {
    let hash = {
        let mut hasher = DefaultHasher::new();
        src.hash(&mut hasher);
        hasher.finish()
    };
    format!("{hash:x}")
}
