use clap::Parser;
use sdql::cli::{filename, run};
use sdql::rs;

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
