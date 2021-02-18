use std::path;
use std::fs;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file {:?}", &args.path))?;

    println!("{}", content);

    Ok(())
}
