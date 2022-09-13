use std::{env, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use loc_stats::get_stats::{get_stats, GetStatsOptions};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    path: String,

    /// Gives the output in JSON format
    #[clap(short, long, action)]
    json: bool,

    /// Ignores files listed in .gitignore. Defaults to true
    #[clap(long, action)]
    gitignore: Option<bool>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let options = GetStatsOptions {
        gitignore: args.gitignore.unwrap_or(true),
    };
    env::set_current_dir(args.path)?;
    let stats = get_stats(PathBuf::from(".".to_owned()).as_path(), &options)?;

    if args.json {
        let json = serde_json::to_string_pretty(&stats)?;
        println!("{}", json);
    } else {
        stats.pretty_output();
    }
    Ok(())
}
