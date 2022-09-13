use std::{
    collections::HashMap,
    fs,
    io::BufRead,
    path::{Path, PathBuf},
};

use crate::langs::{init_languages_hashmap, LangsMap};
use anyhow::Result;
use gitignored::Gitignore;
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
    pub number_of_files: u32,
    pub by_lang: HashMap<&'static str, LangStat>,
}

pub fn get_stats(path: &Path, options: &GetStatsOptions) -> Result<Stats> {
    // configure gitignore
    let gitignore = options.gitignore;
    let mut ig = Gitignore::default();
    let gitignore_path = path.join(".gitignore");
    let globs = fs::read_to_string(gitignore_path).unwrap_or_default();
    let globs: Vec<&str> = globs
        .lines()
        .map(|l| {
            if l.starts_with('/') {
                l.get(1..).unwrap_or_default()
            } else {
                l
            }
        })
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();

    let mut include_file = |p: &Path| {
        if gitignore {
            !ig.ignores(&globs, ig.root.join(p))
        } else {
            true
        }
    };

    // count stats for all files that should be included
    let langs_map = init_languages_hashmap();
    let mut stats = Stats::new();
    for p in get_file_paths(path, &mut include_file)? {
        let line_len = get_file_len(&p)?;
        let lang = get_file_lang(&p, &langs_map).unwrap_or("Other");
        let entry = stats.by_lang.entry(lang).or_default();
        entry.loc += line_len;
        stats.total_loc += line_len;
        stats.number_of_files += 1;
    }

    // calculate percents
    for entry in &mut stats.by_lang {
        entry.1.percent = entry.1.loc as f32 / stats.total_loc as f32 * 100.0;
        // round down to 2 decimal places
        entry.1.percent = (entry.1.percent * 100.0).floor() / 100.0;
    }

    Ok(stats)
}

fn get_file_paths(
    path: &Path,
    include_path: &mut impl FnMut(&Path) -> bool,
) -> Result<Vec<PathBuf>> {
    let mut result = vec![];
    let is_git_dir = path.to_str().unwrap_or_default().contains(".git");
    if path.is_dir() && !is_git_dir && include_path(path) {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                let mut paths = get_file_paths(p.as_path(), include_path)?;
                result.append(&mut paths);
            } else if include_path(&p) {
                result.push(p.to_path_buf());
            }
        }
    }

    Ok(result)
}

fn get_file_len(path: &Path) -> Result<usize> {
    Ok(fs::read(path)?.lines().count())
}

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
