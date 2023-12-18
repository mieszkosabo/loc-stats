use anyhow::{Context, Result};
use loc_stats::get_stats::{get_stats_parallel, GetStatsOptions, LangStat, Stats};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};
use tempfile::tempdir;

#[test]
fn smoke_test() -> Result<()> {
    let dir = tempdir()?;

    let file_path = dir.path().join("test.hs");
    let mut file = File::create(file_path)?;
    write!(file, "-- a\n -- b\n")?;

    let options = GetStatsOptions { gitignore: false };
    assert_eq!(
        get_stats_parallel(dir.path(), &options)?,
        Stats {
            total_loc: 2,
            number_of_files: 1,
            by_lang: HashMap::from([(
                "Haskell",
                LangStat {
                    loc: 2,
                    percent: 100.0
                }
            )])
        }
    );

    Ok(())
}

#[test]
fn deep_dir_tree() -> Result<()> {
    let dir = tempdir()?;
    let mut path = dir.path().to_path_buf();
    for _ in 0..200 {
        path.push("a");
    }

    fs::create_dir_all(&path)?;
    path.push("main.rs");
    fs::write(path, "// wowsers\n")?;

    let options = GetStatsOptions { gitignore: false };
    assert_eq!(
        get_stats_parallel(dir.path(), &options)?,
        Stats {
            total_loc: 1,
            number_of_files: 1,
            by_lang: HashMap::from([(
                "Rust",
                LangStat {
                    loc: 1,
                    percent: 100.0
                }
            )])
        }
    );

    Ok(())
}

#[test]
fn one_million_loc_codebase() -> Result<()> {
    let dir = tempdir()?;
    let mut path = dir.path().to_path_buf();

    for i in 1..=100 {
        path.push(format!("{}.bf", i));
        fs::write(&path, ",.,.,.\n".repeat(10_000))?; // 10k * 100 = 1M, quick math
        path.pop();
    }

    let options = GetStatsOptions { gitignore: false };
    assert_eq!(
        get_stats_parallel(dir.path(), &options)?,
        Stats {
            total_loc: 1_000_000,
            number_of_files: 100,
            by_lang: HashMap::from([(
                "Brainfuck",
                LangStat {
                    loc: 1_000_000,
                    percent: 100.0
                }
            )])
        }
    );

    Ok(())
}

#[test]
fn test_gitignore() -> Result<()> {
    let dir = tempdir()?;

    fs::write(dir.path().join("test.hs"), "-- a\n-- b\n").context("Could not write text file")?;
    fs::write(dir.path().join("test2.js"), "// a\n// b\n").context("Could not write text file")?;

    fs::write(
        dir.path().join(".gitignore"),
        "/test2.js\n\n# this is a comment\n",
    )
    .context("Could not write text file")?;

    let options = GetStatsOptions { gitignore: true };
    assert_eq!(
        get_stats_parallel(dir.path(), &options)?,
        Stats {
            total_loc: 4,
            number_of_files: 2,
            by_lang: HashMap::from([
                (
                    "Haskell",
                    LangStat {
                        loc: 2,
                        percent: 50.0
                    }
                ),
                (
                    "JavaScript",
                    LangStat {
                        loc: 2,
                        percent: 50.0
                    }
                )
            ])
        }
    );

    Ok(())
}
