use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use loc_stats::get_stats::get_stats;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: String,

    /// Gives the output in JSON format
    #[clap(short, long, action)]
    json: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let stats = get_stats(PathBuf::from(args.path).as_path())?;

    if args.json {
        let json = serde_json::to_string_pretty(&stats)?;
        println!("{}", json);
    } else {
        // TODO: pretty printing
        println!("{:?}", stats);
    }
    Ok(())
}
