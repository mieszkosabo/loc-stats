use anyhow::{Context, Result};
use loc_stats::get_stats::{get_stats, GetStatsOptions, LangStat, Stats};
use std::{collections::HashMap, fs, path::PathBuf};
use uuid::Uuid;

// don't run with directly `cargo test`,
// run with scripts/run_tests.sh to run tests and cleanup after them

#[test]
fn smoke_test() -> Result<()> {
    let dir_name = PathBuf::from("tmp")
        .as_path()
        .join(PathBuf::from(Uuid::new_v4().to_string()));
    fs::create_dir_all(&dir_name).context("Could not create test dir")?;
    fs::write(dir_name.as_path().join("test.hs"), "-- a\n-- b\n")
        .context("Could not write text file")?;

    let options = GetStatsOptions { gitignore: false };
    assert_eq!(
        get_stats(dir_name.as_path(), &options)?,
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
    let mut dir_name = PathBuf::from("tmp");
    dir_name.push(Uuid::new_v4().to_string());
    let mut path = dir_name.clone();
    for _ in 0..200 {
        path.push("a");
    }

    fs::create_dir_all(&path)?;
    path.push("main.rs");
    fs::write(path, "// wowsers\n")?;

    let options = GetStatsOptions { gitignore: false };
    assert_eq!(
        get_stats(dir_name.as_path(), &options)?,
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
    let mut dir_name = PathBuf::from("tmp");
    dir_name.push(Uuid::new_v4().to_string());
    fs::create_dir_all(&dir_name)?;
    let mut path = dir_name.clone();

    for i in 1..=100 {
        path.push(format!("{}.bf", i));
        fs::write(&path, ",.,.,.\n".repeat(10_000))?; // 10k * 100 = 1M, quick math
        path.pop();
    }

    let options = GetStatsOptions { gitignore: false };
    assert_eq!(
        get_stats(dir_name.as_path(), &options)?,
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
    let dir_name = PathBuf::from("tmp")
        .as_path()
        .join(PathBuf::from(Uuid::new_v4().to_string()));
    fs::create_dir_all(&dir_name).context("Could not create test dir")?;
    fs::write(dir_name.as_path().join("test.hs"), "-- a\n-- b\n")
        .context("Could not write text file")?;
    fs::write(dir_name.as_path().join("test2.js"), "// a\n// b\n")
        .context("Could not write text file")?;

    fs::write(dir_name.as_path().join(".gitignore"), "test2.js")
        .context("Could not write text file")?;

    let options = GetStatsOptions { gitignore: true };
    assert_eq!(
        get_stats(dir_name.as_path(), &options)?,
        Stats {
            total_loc: 3,
            number_of_files: 2,
            by_lang: HashMap::from([
                (
                    "Haskell",
                    LangStat {
                        loc: 2,
                        percent: 66.66
                    }
                ),
                (
                    "Other",
                    LangStat {
                        loc: 1,
                        percent: 33.33
                    }
                )
            ])
        }
    );

    Ok(())
}
