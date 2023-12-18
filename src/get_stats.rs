use std::sync::Mutex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::langs::{LangsMap, LANGS_MAP};
use anyhow::Result;
use ignore::WalkBuilder;
use serde::Serialize;

pub struct GetStatsOptions {
    pub gitignore: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LangStat {
    pub loc: usize,
    pub percent: f32,
}

impl Default for LangStat {
    fn default() -> Self {
        Self {
            loc: 0,
            percent: 0.0,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Stats {
    pub total_loc: usize,
    pub number_of_files: usize,
    pub by_lang: HashMap<&'static str, LangStat>,
}

pub fn get_stats_sync(path: &Path, options: &GetStatsOptions) -> Result<Stats> {
    let mut paths = Vec::new();

    let sync_walker = WalkBuilder::new(path).git_ignore(options.gitignore).build();
    for result in sync_walker {
        let entry = result?;

        let path = entry.path();
        if path.is_dir() {
            continue;
        } else {
            paths.push(path.to_path_buf());
        }
    }

    let mut stats = Stats::new();
    let mut total_loc = 0;
    let mut total_files = 0;

    paths.iter().for_each(|path| {
        total_files += 1;
        let loc = count_newlines(path).unwrap_or_default();
        let lang = get_file_lang(path, &LANGS_MAP).unwrap_or("Other");
        let entry = stats.by_lang.entry(lang).or_default();

        total_loc += loc;
        entry.loc += loc;
    });

    stats.number_of_files = total_files;
    stats.total_loc = total_loc;

    for entry in &mut stats.by_lang {
        entry.1.percent = entry.1.loc as f32 / stats.total_loc as f32 * 100.0;
        // round down to 2 decimal places
        entry.1.percent = (entry.1.percent * 100.0).floor() / 100.0;
    }

    Ok(stats)
}

pub fn get_stats_parallel(path: &Path, options: &GetStatsOptions) -> Result<Stats> {
    let stats = Mutex::new(Stats::new());

    let walker = WalkBuilder::new(path)
        .git_ignore(options.gitignore)
        .threads(6)
        .build_parallel();
    walker.run(|| {
        Box::new(|result| {
            use ignore::WalkState;

            let entry = match result {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return WalkState::Continue;
                }
            };

            let path = entry.path();
            if path.is_dir() {
                return WalkState::Continue;
            }

            let loc = count_newlines(path).unwrap_or_default();
            let lang = get_file_lang(path, &LANGS_MAP).unwrap_or("Other");

            let mut stats = stats.lock().unwrap();
            stats.total_loc += loc;
            stats.number_of_files += 1;
            let entry = stats.by_lang.entry(lang).or_default();
            entry.loc += loc;

            WalkState::Continue
        })
    });

    let mut stats = stats.into_inner().unwrap();

    for entry in &mut stats.by_lang {
        entry.1.percent = entry.1.loc as f32 / stats.total_loc as f32 * 100.0;
        // round down to 2 decimal places
        entry.1.percent = (entry.1.percent * 100.0).floor() / 100.0;
    }

    Ok(stats)
}

#[inline]
fn count_newlines(path: &Path) -> Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut n = 0;
    let mut v = Vec::new();
    let new_line_byte = 0xA;
    loop {
        v.clear();
        let res = reader.read_until(new_line_byte, &mut v);
        if res.is_err() || res.unwrap() == 0 {
            break;
        }
        n += 1;
    }

    Ok(n)
}

#[inline]
fn get_file_lang(path: &Path, langs_map: &LangsMap) -> Option<&'static str> {
    let ext = path.extension()?;
    Some(langs_map.get(ext.to_str().unwrap_or_default())?)
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total_loc: 0,
            number_of_files: 0,
            by_lang: HashMap::new(),
        }
    }
}
impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
