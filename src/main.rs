use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use loc_stats::get_stats::get_stats;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let stats = get_stats(PathBuf::from(args.path).as_path())?;

    println!("{:?}", stats);
    Ok(())
}
